use crate::{
    bindings::nist_sts::{
        approximate_entropy, block_frenquency, cusum, discrete_fourier_transform, epsilon,
        frequency, linear_complexity, longest_run_of_ones, non_overlapping_template_matchings,
        ApproximateEntropy, BitSequence, BlockFrequency, CumulativeSums, DiscreteFourierTransform,
        Frequency, LinearComplexity, LongestRunOfOnes, NonOverlappingTemplateMatchings,
    },
    key_generations_metadatas::Key,
};
use core::fmt;
use libc::{free, malloc};
use std::{collections::HashMap, ffi::c_void, os, ptr};
use tabled::{
    builder::Builder,
    settings::{Panel, Style},
};
use thiserror::Error;

/// custom error type for RNG metadata operations.
#[derive(Error, Debug)]
pub enum RngMetadatasError {
    #[error("Memory allocation failed (tried to allocate {0} bytes)")]
    MallocError(usize),
    #[error("Couldn't convert the bitvec len to an integer: {0}")]
    BitVectorLen(String),
    #[error("Longest Run Of Ones Error: {0}")]
    LongestRunOfOnes(String),
    #[error("Discrete Fourier Transform: {0}")]
    DiscreteFourierTransform(String),
    #[error("Non Overlapping Template Matching: {0}")]
    NonOverlappingTemplateMatching(String),
    #[error("Linear Complexity: {0}")]
    LinearComplexity(String),
    #[error("Approximate Entropy is Inaccurate!")]
    ApproximateEntropyInaccurate(),
    #[error("Approximate Entropy: {0}")]
    ApproximateEntropy(String),
}

/// struct to display entropy results in a table format.
#[derive(Debug)]
pub struct AliceAndBobTabledRngMetadatas<'a> {
    alice: &'a RngMetadatas<'a>,
    bob: &'a RngMetadatas<'a>,
    block_size: i32,
}

impl fmt::Display for AliceAndBobTabledRngMetadatas<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = Builder::default();
        builder.push_record(["Test Name", "Alice Result", "Bob Result"]);

        macro_rules! add_entropy_test {
            ($name:expr, $alice_method:expr, $bob_method:expr) => {
                builder.push_record([
                    $name,
                    format!("{:.7}", $alice_method).as_str(),
                    format!("{:.7}", $bob_method).as_str(),
                ]);
            };
        }

        macro_rules! add_statistical_test {
            ($name:expr, $alice_method:expr, $bob_method:expr) => {
                builder.push_record([
                    $name,
                    format!(
                        "{:.7}",
                        $alice_method.expect("Statistical test failed").p_value
                    )
                    .as_str(),
                    format!(
                        "{:.7}",
                        $bob_method.expect("Statistical test failed").p_value
                    )
                    .as_str(),
                ]);
            };
        }

        // entropy tests
        add_entropy_test!("Per-Bit Entropy", self.alice.entropy(), self.bob.entropy());
        add_entropy_test!(
            "Per-Byte Entropy",
            self.alice.byte_entropy(),
            self.bob.byte_entropy()
        );
        add_entropy_test!(
            "Per-Bit Min-Entropy",
            self.alice.min_entropy(),
            self.bob.min_entropy()
        );
        add_entropy_test!(
            "Per-Byte Min-Entropy",
            self.alice.byte_min_entropy(),
            self.bob.byte_min_entropy()
        );

        // NIST 800-22 statistical tests
        add_statistical_test!("Frequency", self.alice.frequency(), self.bob.frequency());
        add_statistical_test!(
            "Block Frequency",
            self.alice.block_frequency(self.block_size),
            self.bob.block_frequency(self.block_size)
        );
        add_statistical_test!(
            "Longest Run Of Ones",
            self.alice.longest_run_of_ones(),
            self.bob.longest_run_of_ones()
        );
        add_statistical_test!(
            "Discrete Fourier Transform",
            self.alice.discrete_fourier_transform(),
            self.bob.discrete_fourier_transform()
        );
        add_statistical_test!(
            "Non Overlapping Template Matching",
            self.alice
                .non_overlapping_template_matching(self.block_size),
            self.bob.non_overlapping_template_matching(self.block_size)
        );
        add_statistical_test!(
            "Linear Complexity",
            self.alice.linear_complexity(self.block_size),
            self.bob.linear_complexity(self.block_size)
        );
        add_statistical_test!(
            "Approximate Entropy",
            self.alice.approximate_entropy(self.block_size),
            self.bob.approximate_entropy(self.block_size)
        );
        add_statistical_test!(
            "Cumulative Sums",
            self.alice.cumulative_sum(),
            self.bob.cumulative_sum()
        );

        let mut table = builder.build();
        table.with(Style::modern_rounded());
        table.with(Panel::horizontal(
            1,
            "                        Entropy Results",
        ));
        table.with(Panel::horizontal(
            6,
            "                       NIST 800-22 Suite",
        ));

        write!(f, "{table}")
    }
}

impl<'a> AliceAndBobTabledRngMetadatas<'a> {
    pub const fn new(alice: &'a RngMetadatas, bob: &'a RngMetadatas, block_size: i32) -> Self {
        Self {
            alice,
            bob,
            block_size,
        }
    }
}

/// struct to perform various statistical tests and entropy calculations on a key.
#[derive(Debug)]
pub struct RngMetadatas<'a> {
    key: &'a Key,
}

impl<'a> RngMetadatas<'a> {
    pub const fn new(key: &'a Key) -> Self {
        Self { key }
    }

    /// converts the key bytes into a vector of bits.
    fn byte_to_bits(&self) -> Vec<u8> {
        let mut bitvec: Vec<u8> = Vec::with_capacity(self.key.len());
        for byte in self.key.bytes() {
            for i in (0..8).rev() {
                bitvec.push((byte >> i) & 1);
            }
        }
        bitvec
    }

    /// prepares the data for statistical tests by converting the key to a bit sequence.
    unsafe fn prepare_data(&self) -> Result<*mut BitSequence, RngMetadatasError> {
        let n = self.key.len();
        let bitvec = self.byte_to_bits();
        epsilon = malloc(n).cast::<u8>();
        if epsilon.is_null() {
            Err(RngMetadatasError::MallocError(n))
        } else {
            ptr::copy_nonoverlapping(bitvec.as_ptr(), epsilon, bitvec.len());
            Ok(epsilon)
        }
    }

    /// Performs the frequency test.
    pub fn frequency(&self) -> Result<frequency, RngMetadatasError> {
        unsafe {
            epsilon = self.prepare_data()?;
            let length = os::raw::c_int::try_from(self.key.len())
                .map_err(|e| RngMetadatasError::BitVectorLen(e.to_string()))?;
            let result = Frequency(length);
            free(epsilon.cast::<c_void>());
            Ok(result)
        }
    }

    /// performs the block frequency test.
    pub fn block_frequency(&self, block_size: i32) -> Result<block_frenquency, RngMetadatasError> {
        unsafe {
            epsilon = self.prepare_data()?;
            let length = os::raw::c_int::try_from(self.key.len())
                .map_err(|e| RngMetadatasError::BitVectorLen(e.to_string()))?;
            let result = BlockFrequency(block_size, length);
            free(epsilon.cast::<c_void>());
            Ok(result)
        }
    }

    /// performs the longest run of ones test.
    pub fn longest_run_of_ones(&self) -> Result<longest_run_of_ones, RngMetadatasError> {
        unsafe {
            epsilon = self.prepare_data()?;
            let length = os::raw::c_int::try_from(self.key.len())
                .map_err(|e| RngMetadatasError::BitVectorLen(e.to_string()))?;
            let result = LongestRunOfOnes(length);
            free(epsilon.cast::<c_void>());
            if result.valid_run {
                Ok(result)
            } else {
                Err(RngMetadatasError::LongestRunOfOnes(
                    "Probably because of an early return!".to_string(),
                ))
            }
        }
    }

    /// performs the discrete Fourier transform test.
    pub fn discrete_fourier_transform(
        &self,
    ) -> Result<discrete_fourier_transform, RngMetadatasError> {
        unsafe {
            epsilon = self.prepare_data()?;
            let length = os::raw::c_int::try_from(self.key.len())
                .map_err(|e| RngMetadatasError::BitVectorLen(e.to_string()))?;
            let result = DiscreteFourierTransform(length);
            free(epsilon.cast::<c_void>());
            if result.valid_run {
                Ok(result)
            } else {
                Err(RngMetadatasError::DiscreteFourierTransform(
                    "Probably because of an early return!".to_string(),
                ))
            }
        }
    }

    /// performs the non-overlapping template matching test.
    pub fn non_overlapping_template_matching(
        &self,
        block_size: i32,
    ) -> Result<non_overlapping_template_matchings, RngMetadatasError> {
        unsafe {
            epsilon = self.prepare_data()?;
            let length = os::raw::c_int::try_from(self.key.len())
                .map_err(|e| RngMetadatasError::BitVectorLen(e.to_string()))?;
            let result = NonOverlappingTemplateMatchings(block_size, length);
            free(epsilon.cast::<c_void>());
            if result.valid_run {
                Ok(result)
            } else {
                Err(RngMetadatasError::NonOverlappingTemplateMatching(
                    "Probably because of an early return!".to_string(),
                ))
            }
        }
    }

    /// performs the linear complexity test.
    pub fn linear_complexity(
        &self,
        block_size: i32,
    ) -> Result<linear_complexity, RngMetadatasError> {
        unsafe {
            epsilon = self.prepare_data()?;
            let length = os::raw::c_int::try_from(self.key.len())
                .map_err(|e| RngMetadatasError::BitVectorLen(e.to_string()))?;
            let result = LinearComplexity(block_size, length);
            free(epsilon.cast::<c_void>());
            if result.valid_run {
                Ok(result)
            } else {
                Err(RngMetadatasError::LinearComplexity(
                    "Probably because of an early return!".to_string(),
                ))
            }
        }
    }

    /// performs the approximate entropy test.
    pub fn approximate_entropy(
        &self,
        block_size: i32,
    ) -> Result<approximate_entropy, RngMetadatasError> {
        unsafe {
            epsilon = self.prepare_data()?;
            let length = os::raw::c_int::try_from(self.key.len())
                .map_err(|e| RngMetadatasError::BitVectorLen(e.to_string()))?;
            let result = ApproximateEntropy(block_size, length);
            free(epsilon.cast::<c_void>());
            if result.innacurate {
                Err(RngMetadatasError::ApproximateEntropyInaccurate())
            } else if result.valid_run {
                Ok(result)
            } else {
                Err(RngMetadatasError::ApproximateEntropy(
                    "Probably because of an early return!".to_string(),
                ))
            }
        }
    }

    /// performs the cumulative sum test.
    pub fn cumulative_sum(&self) -> Result<cusum, RngMetadatasError> {
        unsafe {
            epsilon = self.prepare_data()?;
            let length = os::raw::c_int::try_from(self.key.len())
                .map_err(|e| RngMetadatasError::BitVectorLen(e.to_string()))?;
            let result = CumulativeSums(length);
            free(epsilon.cast::<c_void>());
            Ok(result)
        }
    }

    /// calculates the Shannon entropy on bit level.
    pub fn entropy(&self) -> f64 {
        let number_of_1s = f64::from(
            self.key
                .bytes()
                .iter()
                .fold(0, |acc, byte| acc + byte.count_ones()),
        );
        let key_len = self.key.len();
        let p_1 = number_of_1s / key_len as f64;
        let p_0 = 1f64 - p_1;
        if p_0 == 0f64 || p_1 == 0f64 {
            return 0f64;
        }
        (-p_1).mul_add(p_1.log2(), -(p_0 * p_0.log2()))
    }

    /// normalizes a byte value.
    pub fn normalized_byte(&self, non_normalized_byte: f64) -> f64 {
        let len_bytes_f64 = self.key.len_bytes() as f64;
        non_normalized_byte / len_bytes_f64.log2()
    }

    /// calculates the Shannon entropy on byte level.
    pub fn byte_entropy(&self) -> f64 {
        let key_len = self.key.len_bytes() as f64;
        let mut frequency = HashMap::new();
        for byte in self.key.bytes() {
            *frequency.entry(byte).or_insert(0) += 1;
        }
        if frequency.len() == 1 {
            return 0.0;
        }
        -frequency
            .values()
            .map(|freq| f64::from(*freq) / key_len)
            .fold(0.0, |sum, x_n| x_n.mul_add(x_n.log2(), sum))
    }

    /// calculates the min-entropy on bit level.
    pub fn min_entropy(&self) -> f64 {
        let number_of_1s = self
            .key
            .bytes()
            .iter()
            .fold(0f64, |acc, byte| acc + f64::from(byte.count_ones()));
        let key_len = self.key.len();
        let p_1 = number_of_1s / key_len as f64;
        let p_0 = 1f64 - p_1;
        if p_0 == 0.0 || p_1 == 0.0 {
            return 0.0;
        }
        let v_0 = -p_0.log2();
        let v_1 = -p_1.log2();
        v_0.min(v_1)
    }

    /// calculates the min-entropy on byte level.
    pub fn byte_min_entropy(&self) -> f64 {
        let key_len = self.key.len_bytes() as f64;
        let mut frequency = HashMap::new();
        for byte in self.key.bytes() {
            *frequency.entry(byte).or_insert(0) += 1;
        }
        if frequency.len() == 1 {
            return 0.0;
        }
        frequency
            .values()
            .map(|freq| f64::from(*freq) / key_len)
            .map(|p_x| -p_x.log2())
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
}
