#include "../include/defs.h"
#include <stdio.h>

/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
                   G L O B A L   D A T A  S T R U C T U R E S
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

extern BitSequence *epsilon;          // BIT STREAM
extern TP tp;                         // TEST PARAMETER STRUCTURE
extern FILE *stats[NUMOFTESTS + 1];   // FILE OUTPUT STREAM
extern FILE *results[NUMOFTESTS + 1]; // FILE OUTPUT STREAM
extern FILE *freqfp;                  // FILE OUTPUT STREAM
extern FILE *summary;                 // FILE OUTPUT STREAM
extern int testVector[NUMOFTESTS + 1];

extern char generatorDir[NUMOFGENERATORS][20];
extern char testNames[NUMOFTESTS + 1][32];

extern frequency Frequency(int n);
extern block_frenquency BlockFrequency(int M, int n);
extern longest_run_of_ones LongestRunOfOnes(int n);
extern discrete_fourier_transform DiscreteFourierTransform(int n);
extern non_overlapping_template_matchings
NonOverlappingTemplateMatchings(int m, int n);
extern linear_complexity LinearComplexity(int M, int n);
extern approximate_entropy ApproximateEntropy(int m, int n);
extern cusum CumulativeSums(int n);
