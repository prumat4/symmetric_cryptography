#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define L1_DEG 25
#define L2_DEG 26
#define L3_DEG 27
#define DUMMY_STRING "00111100011010100111011000000101011111001011010000101001001111010011101111110010011010000011011110100001011001001000010010111010001110000100010100110111100011010110100111100111001110000010111011111111010110010110011010101110110111011010000101000010010000010111010100010111010010001101010001010111000010111100011110100011100011011010010011001001011100001101100100110000011110101100100001000100000101011111111101010101110011001000111001110110110001110001000010101001011010010101011011110000111101101000111001111101111011010101100110001001010000010010001100100001110001010100010011011111100111011111010110000001111000010000111110101111010111111011101001011010000100001010101101000100010011101011111111101011000000001110011011101100110110001010010011001101100110100011011000111000111110100110100010101010100011111010111010110101100011000000010000010111000101100001010111001000000100010100101011000100110110101100111010110111001110001001001111010011011110010101011100100110110111110111110111010110100010110111010110101011101011100100010101011000100101111011110101010100101101010111011111100001101111010100011011011011111100010100110111100110111111011011101010011011111110100101001100111010101101000010111010010000111001110001000001110011010111111111101111111000110011001010011110001111100011110100110100001010001111000100110011000101000011100001000001110001001100011000010111111010001100101110011110001111101101101001010010011100101100111110010100101100110001111101011001101111001011001001001110000110000110000001001101101100110001000101010010001110000110010001000001000110101000011100100101101011101101111101000100110000100101000110001101010011100101011110001111000100111110010111010000111101010001100101011111100110110011110001110000100101110101110011100110111101011100111101001000101000001100100001101110000101101011011001010101100110110110111000111011101111001110010010000010111010001111000000011001010010100000110001100000010101000111110101000100001100100101000011000010100100001110110011101101111010001100100101001011011111001110001011010010110011"
#define STRING_LEN 2048

typedef struct {
    uint32_t filling;
    uint32_t mask_for_bit_to_set;
    uint8_t max_bit_n;
    uint32_t recurvia;
} LFSR;

LFSR init_LFSR(uint32_t recurv_coefs, uint8_t poly_deg) {
    LFSR lfsr;
    lfsr.max_bit_n = poly_deg - 1;
    lfsr.recurvia = recurv_coefs;
    lfsr.mask_for_bit_to_set = (1 << lfsr.max_bit_n);
    return lfsr;
}

uint8_t* generate_from_fill(LFSR* lfsr, uint32_t _filling, uint64_t length) {
    lfsr->filling = _filling;
    uint8_t* feedback = (uint8_t*)malloc(length * sizeof(uint8_t));

    for (uint64_t i = 0; i < length; ++i) {
        feedback[i] = (uint8_t)(lfsr->filling & 1);
        lfsr->filling = (lfsr->filling >> 1) ^ ((__builtin_popcount(lfsr->filling & lfsr->recurvia) & 1u) << lfsr->max_bit_n);
    }

    return feedback;
}

typedef struct {
    LFSR L1;
    LFSR L2;
    LFSR L3;
} Geffe;

Geffe init_Geffe(LFSR* L1, LFSR* L2, LFSR* L3) {
    Geffe geffe;
    geffe.L1 = *L1;
    geffe.L2 = *L2;
    geffe.L3 = *L3;
    return geffe;
}

uint8_t* generate(Geffe* geffe, uint32_t init_l1, uint32_t init_l2, uint32_t init_l3, size_t length) {
    uint8_t* res = (uint8_t*)malloc(length * sizeof(uint8_t));
    uint8_t* X = generate_from_fill(&geffe->L1, init_l1, length);
    uint8_t* Y = generate_from_fill(&geffe->L2, init_l2, length);
    uint8_t* S = generate_from_fill(&geffe->L3, init_l3, length);

    for (size_t i = 0; i < length; ++i) {
        res[i] = (S[i] == 1) ? X[i] : Y[i];
    }

    free(X);
    free(Y);
    free(S);

    return res;
}

void find_candidates(LFSR* lfsr, uint8_t* r_seq, size_t N_req, uint32_t C, uint32_t* best_candidate, size_t* best_R) {
    uint64_t cyclen = ((uint64_t)1 << lfsr->max_bit_n) + (uint64_t)N_req;
    uint32_t curr_candidate = 1u;
    uint8_t* generated_seq = generate_from_fill(lfsr, curr_candidate, cyclen);
    *best_candidate = 0;
    *best_R = SIZE_MAX;

    for (size_t j = 0; j < ((uint64_t)1 << lfsr->max_bit_n); ++j) {
        size_t R = 0;
        for (size_t i = 0; i < N_req; ++i) {
            R += (generated_seq[j + i] ^ r_seq[i]);
        }

        if (R < C && R < *best_R) {
            *best_candidate = curr_candidate;
            *best_R = R;
        }

        curr_candidate = (curr_candidate >> 1) ^ ((uint32_t)generated_seq[lfsr->max_bit_n + j] << (lfsr->max_bit_n - 1));
    }

    free(generated_seq);
}

int main() {
    clock_t start, stop;
    start = clock();

    uint32_t L1_rec = ((1u << 3) ^ 1);
    uint32_t L2_rec = ((1u << 6) ^ (1u << 2) ^ (1u << 1) ^ 1u);
    uint32_t L3_rec = ((1u << 5) ^ (1u << 2) ^ (1u << 1) ^ 1u);

    uint32_t N = strlen(DUMMY_STRING);
    uint8_t r_seq[N];
    for (size_t i = 0; i < N; ++i) {
        r_seq[i] = DUMMY_STRING[i] - '0';
    }

    uint32_t N1_req = 222;
    uint32_t C1 = 71;
    uint32_t N2_req = 229;
    uint32_t C2 = 74;

    LFSR L1 = init_LFSR(L1_rec, L1_DEG);
    LFSR L2 = init_LFSR(L2_rec, L2_DEG);
    LFSR L3 = init_LFSR(L3_rec, L3_DEG);

    uint32_t L1_candidate, L2_candidate, L3_candidate;
    size_t best_R_L1, best_R_L2;

    find_candidates(&L1, r_seq, N1_req, C1, &L1_candidate, &best_R_L1);
    find_candidates(&L2, r_seq, N2_req, C2, &L2_candidate, &best_R_L2);

    uint64_t cyclen = ((uint64_t)1 << L3_DEG) + (uint64_t)N;
    uint32_t curr_candidate = 1u;
    uint8_t* L3_seq = generate_from_fill(&L3, curr_candidate, cyclen);
    uint8_t* L1_seq = generate_from_fill(&L1, L1_candidate, N);
    uint8_t* L2_seq = generate_from_fill(&L2, L2_candidate, N);

    for (size_t j = 0; j < ((uint64_t)1 << L3_DEG); ++j) {
        int found = 1;
        for (size_t i = 0; i < N; ++i) {
            if (((L3_seq[j + i] & L1_seq[i]) ^ ((1u ^ L3_seq[j + i]) & L2_seq[i])) != r_seq[i]) {
                found = 0;
                break;
            }
        }

        if (found) {
            L3_candidate = curr_candidate;
            break;
        }

        curr_candidate = (curr_candidate >> 1) ^ ((uint32_t)L3_seq[L3_DEG + j] << (L3_DEG - 1));
    }

    free(L3_seq);
    free(L1_seq);
    free(L2_seq);

    printf("L1 candidate: %10u ", L1_candidate);
    for (int i = L1_DEG - 1; i >= 0; i--) {
        printf("%d", (L1_candidate >> i) & 1);
    }
    printf("\n");

    printf("L2 candidate: %10u ", L2_candidate);
    for (int i = L2_DEG - 1; i >= 0; i--) {
        printf("%d", (L2_candidate >> i) & 1);
    }
    printf("\n");

    printf("L3 candidate: %10u ", L3_candidate);
    for (int i = L3_DEG - 1; i >= 0; i--) {
        printf("%d", (L3_candidate >> i) & 1);
    }
    printf("\n");

    Geffe generator = init_Geffe(&L1, &L2, &L3);
    uint8_t* test_gen = generate(&generator, L1_candidate, L2_candidate, L3_candidate, STRING_LEN);

    printf("Generated sequence:\n");
    for (size_t i = 0; i < STRING_LEN; ++i) {
        printf("%d", test_gen[i]);
    }
    printf("\n");

    printf("Expected sequence:\n");
    for (size_t i = 0; i < N; ++i) {
        printf("%d", r_seq[i]);
    }
    printf("\n");

    free(test_gen);

    stop = clock();
    printf("\nExecution time: %lf seconds\n", (double)(stop - start) / CLOCKS_PER_SEC);

    return 0;
}
