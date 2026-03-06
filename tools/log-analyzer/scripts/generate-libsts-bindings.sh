#!/bin/sh

VARIABLE_LIST="epsilon"
STRUCT_LIST="approximate_entropy|block_frenquency|cusum|discrete_fourier_transform|frequency|linear_complexity|longest_run_of_ones|non_overlapping_template_matchings"
FUNCTION_LIST="ApproximateEntropy|BitSequence|BlockFrequency|CumulativeSums|DiscreteFourierTransform|Frequency|LinearComplexity|LongestRunOfOnes|NonOverlappingTemplateMatchings"

result=$(bindgen --allowlist-type "$STRUCT_LIST" --allowlist-function "$FUNCTION_LIST" --allowlist-var "$VARIABLE_LIST" bindings/libsts_bindgen_headers.h)

echo "#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
" >src/bindings/nist_sts.rs

echo "$result" >>src/bindings/nist_sts.rs

cargo clean
