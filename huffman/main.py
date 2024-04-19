import time
import queue
MAX_SIZE = 100


class HuffmanTreeNode:
    def __init__(self, character, frequency):
        # Stores character
        self.data = character

        # Stores frequency of the character
        self.freq = frequency

        # Left child of the current node
        self.left = None

        # Right child of the current node
        self.right = None

    def __lt__(self, other):
        return self.freq < other.freq

# Custom comparator class


class Compare:
    def __call__(self, a, b):
        # Defining priority on the basis of frequency
        return a.freq > b.freq

# Function to generate Huffman Encoding Tree


def generateTree(pq):
    # We keep on looping till only one node remains in the Priority Queue
    while pq.qsize() != 1:
        # Node which has least frequency
        left = pq.get()

        # Node which has least frequency
        right = pq.get()

        # A new node is formed with frequency left.freq + right.freq
        # We take data as '$' because we are only concerned with the frequency
        node = HuffmanTreeNode('$', left.freq + right.freq)
        node.left = left
        node.right = right

        # Push back node created to the Priority Queue
        pq.put(node)

    return pq.get()

# Function to print the huffman code for each character.
# It uses arr to store the codes


def printCodes(root, arr, top):
    # Assign 0 to the left node and recur
    if root.left:
        arr[top] = 0
        printCodes(root.left, arr, top + 1)

    # Assign 1 to the right node and recur
    if root.right:
        arr[top] = 1
        printCodes(root.right, arr, top + 1)

    # If this is a leaf node, then we print root.data
    # We also print the code for this character from arr
    if not root.left and not root.right:
        print(root.data, end=',')
        for i in range(top):
            print(arr[i], end='')
        print()


def HuffmanCodes(data, freq, size):
    # Declaring priority queue using custom comparator
    pq = queue.PriorityQueue()

    # Populating the priority queue
    for i in range(size):
        newNode = HuffmanTreeNode(data[i], freq[i])
        pq.put(newNode)

    # Generate Huffman Encoding Tree and get the root node
    root = generateTree(pq)

    # Print Huffman Codes
    arr = [0] * MAX_SIZE
    top = 0
    printCodes(root, arr, top)


def process_lines_to_dict(filename, max_lines=2**17):
    if max_lines <= 0:
        raise ValueError("max_lines must be a positive integer")

    words = []
    ns = []
    with open(filename, 'r') as f:
        for _ in range(max_lines):
            line = f.readline().strip()  # Read and strip leading/trailing whitespaces
            if not line:
                break  # Reached EOF before max_lines

            try:
                # Split the line by the middle comma
                key, value_str = line.split(",", 1)
                # Convert the second part to a number
                ns.append(int(value_str))
                words.append(key)
            except (ValueError, IndexError):
                # Handle potential errors like missing comma or invalid format
                pass
    return (words, ns)


def load_file_freq(filename):
    char_map = {}
    try:
        with open(filename, 'r') as f:
            for line in f:
                for char in line:
                    char_map[char] = char_map.get(char, 0) + 1
    except FileNotFoundError:
        print(f"Error: File '{filename}' not found.")
        return None
    return char_map


# Driver Code
if __name__ == '__main__':
    #data, freq = process_lines_to_dict("../unigram_freq.csv")
    d = load_file_freq("../cantrbry/alice29.txt")
    data = [ord(c) for c in list(d.keys())]
    freq = list(d.values())
    size = len(data)

    HuffmanCodes(data, freq, size)
