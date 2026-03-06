/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
                       D E B U G G I N G  A I D E S
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

#include "config.h"
#include <stdbool.h>

/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
                              M A C R O S
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

#define MAX(x, y) ((x) < (y) ? (y) : (x))
#define MIN(x, y) ((x) > (y) ? (y) : (x))
#define isNonPositive(x) ((x) <= 0.e0 ? 1 : 0)
#define isPositive(x) ((x) > 0.e0 ? 1 : 0)
#define isNegative(x) ((x) < 0.e0 ? 1 : 0)
#define isGreaterThanOne(x) ((x) > 1.e0 ? 1 : 0)
#define isZero(x) ((x) == 0.e0 ? 1 : 0)
#define isOne(x) ((x) == 1.e0 ? 1 : 0)

/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
                         G L O B A L  C O N S T A N T S
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

#define ALPHA 0.01            /* SIGNIFICANCE LEVEL */
#define MAXNUMOFTEMPLATES 148 /* APERIODIC TEMPLATES: 148=>temp_length=9 */
#define NUMOFTESTS 15         /* MAX TESTS DEFINED  */
#define NUMOFGENERATORS 10    /* MAX PRNGs */
#define MAXFILESPERMITTEDFORPARTITION 148
#define TEST_FREQUENCY 1
#define TEST_BLOCK_FREQUENCY 2
#define TEST_CUSUM 3
#define TEST_RUNS 4
#define TEST_LONGEST_RUN 5
#define TEST_RANK 6
#define TEST_FFT 7
#define TEST_NONPERIODIC 8
#define TEST_OVERLAPPING 9
#define TEST_UNIVERSAL 10
#define TEST_APEN 11
#define TEST_RND_EXCURSION 12
#define TEST_RND_EXCURSION_VAR 13
#define TEST_SERIAL 14
#define TEST_LINEARCOMPLEXITY 15

/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
                   G L O B A L   D A T A  S T R U C T U R E S
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

typedef unsigned char BitSequence;

typedef struct _testParameters {
  int n;
  int blockFrequencyBlockLength;
  int nonOverlappingTemplateBlockLength;
  int overlappingTemplateBlockLength;
  int serialBlockLength;
  int linearComplexitySequenceLength;
  int approximateEntropyBlockLength;
  int numOfBitStreams;
} TP;

typedef struct frequency_s {
  double p_value;
  bool passed;
} frequency;

typedef struct block_frenquency_s {
  double chi_squared;
  double p_value;
  bool passed;
} block_frenquency;

typedef struct longest_run_of_ones_s {
  bool valid_run;
  double chi_squared;
  unsigned int nu[7];
  double p_value;
  bool passed;
} longest_run_of_ones;

typedef struct discrete_fourier_transform_s {
  bool valid_run;
  double percentile;
  double n_l;
  double n_o;
  double d;
  double p_value;
  bool passed;
} discrete_fourier_transform;

typedef struct non_overlapping_template_matchings_s {
  bool valid_run;
  double chi_squared;
  double p_value;
  bool passed;
} non_overlapping_template_matchings;

typedef struct linear_complexity_s {
  bool valid_run;
  double chi_squared;
  unsigned int nu[7];
  double p_value;
  bool passed;
} linear_complexity;

typedef struct approximate_entropy_s {
  bool valid_run;
  double chi_squared;
  double p_value;
  double phi_m;
  double phi_mp1;
  double apen;
  bool innacurate;
  bool passed;
} approximate_entropy;

typedef struct cusum_s {
  double p_value;
  bool passed;
} cusum;
