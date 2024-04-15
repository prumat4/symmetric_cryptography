ALPHABET = "абвгдежзийклмнопрстуфхцчшщьыэюя"
M = len(ALPHABET) * len(ALPHABET)

PROBS = [0.08143, 0.01667, 0.04604, 0.01632, 0.03084, 0.08027, 0.00884,
      0.01507, 0.07563, 0.01200, 0.03374, 0.03952, 0.03270, 0.06503, 0.11143,
      0.02931, 0.04774, 0.05482, 0.06829, 0.02647, 0.00310, 0.00827, 0.00455,
      0.01458, 0.00681, 0.00330, 0.01808, 0.01752, 0.00425, 0.00735, 0.01818,
      0.00036]

BIGRAM_RING = {}
BIGRAMS = ['aa'] * (M)
MOST_FREQUENT_BIGRAMS = ['ст', 'но', 'то', 'на', 'ен']

EXPECTED_I = sum([(p ** 2) for p in PROBS])

INPUT_FILE = "../text_files/affine_bigram_analysis/input.txt"
OUTPUT_FILE = "../text_files/affine_bigram_analysis/output.txt"

def adjust_symbol(c):
    if 'а' <= c and c <= 'я':
        return c
    elif c <= 'Я' and c >= 'А':
        return c.lower()
    elif c == 'Ё' or c == 'ё':
        return 'е'
    else:
        return ""
        
def preprocess_text(input_text):
    result_text = [adjust_symbol(character) for character in input_text]
    result_text = ' '.join(''.join(result_text).split())
    return result_text

def read_text(filepath):
    with open(filepath, "r", encoding='utf-8') as file_handle:
        file_content = file_handle.read()
    return preprocess_text(file_content)

def gcd_extended(a, b):
    x0, x1, y0, y1 = 1, 0, 0, 1
    while b != 0:
        q, a, b = a // b, b, a % b
        x0, x1 = x1, x0 - q * x1
        y0, y1 = y1, y0 - q * y1
    return a, x0, y0

def solve_linear(multiplier, constant, mod_value):
    multiplier, constant = multiplier % mod_value, constant % mod_value
    gcd, x, _ = gcd_extended(multiplier, mod_value)
    if gcd == 1:
        return [(x * constant) % mod_value]
    if constant % gcd == 0:
        m, c, mod = multiplier // gcd, constant // gcd, mod_value // gcd
        _, x, _ = gcd_extended(m, mod)
        return [(x * c + i * mod) % mod_value for i in range(gcd)]
    return []

def count_bigrams_wo_i(input_text):
    bigram_count = {}
    for i in range(1, len(input_text), 2):
        bigram = input_text[i - 1:i + 1]
        bigram_count[bigram] = bigram_count.get(bigram, 0) + 1
    return {k: v for k, v in sorted(bigram_count.items(), key=lambda item: -item[1])}

def get_best_bigrams(input_text, top_n):
    bigrams_freq = count_bigrams_wo_i(input_text)
    return list(bigrams_freq.keys())[:top_n]

def generate_keys(selected_keys):
    potential_keys = []
    for first_i in range(5):
        for second_i in range(5):
            if first_i == second_i:
                continue
            X_first = BIGRAM_RING[MOST_FREQUENT_BIGRAMS[first_i]]
            X_second = BIGRAM_RING[MOST_FREQUENT_BIGRAMS[second_i]]

            for first_j in range(5):
                for second_j in range(5):
                    if first_j == second_j:
                        continue
                    Y_first = BIGRAM_RING[selected_keys[first_j]]
                    Y_second = BIGRAM_RING[selected_keys[second_j]]

                    linear_solutions = solve_linear(X_first - X_second, Y_first - Y_second, M)
                    for coef in linear_solutions:
                        potential_keys.append((coef, (Y_first - coef * X_first) % M))

    return potential_keys


def decrypt_bigram(encrypted, inverse_a, shift_b):
    decrypted_value = BIGRAM_RING[encrypted]
    return BIGRAMS[(inverse_a * (decrypted_value - shift_b)) % M]

def try_decrypt_text(encrypted_text, decryption_key):
    factor_a, shift_b = decryption_key
    gcd_value, inverse_a, _ = gcd_extended(factor_a, M)
    if gcd_value != 1:
        return "invalid key!"
    
    decrypted_text = "".join(
        decrypt_bigram(encrypted_text[i - 1:i + 1], inverse_a, shift_b)
        for i in range(1, len(encrypted_text), 2)
    )
    return decrypted_text

def count_chars(input_text):
    char_frequency = {}
    for char in input_text:
        char_frequency[char] = char_frequency.get(char, 0) + 1
    return sorted(char_frequency.items(), key=lambda item: -item[1])

def coincidence_index(input_text):
    frequency_sum = sum(
        input_text.count(char) * (input_text.count(char) - 1)
        for char in set(input_text)
    )
    n = len(input_text)
    return frequency_sum / (n * (n - 1)) if n > 1 else 0

def compute_text_rate(text_analysis, complexity_level=5):
    frequencies = count_chars(text_analysis)
    rank_size = 4
    score = 0.0

    top_characters = {frequencies[i][0] for i in range(-1, -rank_size - 1, -1)}
    score += 1 if {'о', 'е', 'а'} & top_characters else 0

    if complexity_level <= 1:
        return score

    bottom_characters = {frequencies[i][0] for i in range(rank_size)}
    score += 1 if {'ф', 'ц', 'щ'} & bottom_characters else 0

    if complexity_level <= 2:
        return score

    deviation = abs(coincidence_index(text_analysis) - EXPECTED_I) * 200
    score -= deviation

    return score if complexity_level <= 3 else score


def main():
    global ALPHABET, m, M, MOST_FREQUENT_BIGRAMS, RING, BIGRAM_RING, BIGRAMS, PROBS, EXPECTED_I, INPUT_FILE
    n = 5
    
    RING = {char: pos for pos, char in enumerate(ALPHABET)}
    for i, char1 in enumerate(ALPHABET):
        for j, char2 in enumerate(ALPHABET):
            t = i * len(ALPHABET) + j
            bigram = char1 + char2
            BIGRAM_RING[bigram] = t
            BIGRAMS[t] = bigram
    
    text = read_text(INPUT_FILE)
    best_5 = get_best_bigrams(text, 5)
    print("Best 5 bigrams:", best_5)

    keys = list(dict.fromkeys(generate_keys(best_5)))
    keys_rated = []
    for key in keys:
        open_text = try_decrypt_text(text, key)
        if open_text == "invalid key!":
            continue
        keys_rated.append((key, compute_text_rate(open_text)))

    sorted_by_rate = sorted(keys_rated, key=lambda entry: -entry[1])
    print(f"Top {n} rated:")
    for key, rate in sorted_by_rate[:n]:
        print(f"{rate} : {key}")

    decrypted_text = try_decrypt_text(text, sorted_by_rate[0][0])
    print(decrypted_text)
    with open(OUTPUT_FILE, "w", encoding="utf-8") as out_file:
        out_file.write(decrypted_text)

if __name__ == "__main__":
    main()