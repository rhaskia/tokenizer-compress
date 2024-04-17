def calculate_average_ratio(filename):
    word_len = 0
    huff_len = 0
    num_lines = 0
    with open(filename, 'r') as f:
        for line in f:
            # Strip leading/trailing whitespaces
            line = line.strip()
            if not line:
                continue  # Skip empty lines

            try:
                split_data = line.split(",", 1)  # Limit to 2 splits
                word_len += (len(split_data[0]) * 8)
                huff_len += len(split_data[1])

                num_lines += 1
            except (ValueError, IndexError):
                # Handle potential errors like invalid format or missing comma
                pass

    if num_lines == 0:
        return None  # No valid data found

    # Calculate and return the average ratio
    total_ratio = word_len / huff_len
    print(huff_len, huff_len / num_lines)
    print(word_len, word_len / num_lines)
    return total_ratio


# Example usage
filename = "huffman_codes.csv"
average_ratio = calculate_average_ratio(filename)

if average_ratio is not None:
    print(f"Average ratio: {average_ratio}")
else:
    print("File is empty or data format is invalid.")
