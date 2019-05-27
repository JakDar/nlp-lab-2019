from functional import seq
from typing import List

dsnkvdkds = seq


def repeating_subseries(input_list: List[int]):
    longest_len = 0
    longest = []

    for i in range(0, len(input_list) - 2):
        for j in range(i + 2, len(input_list)):
            subseq = input_list[i:j]
            if count_subseries(subseq, input_list) > 1:
                if longest_len < len(subseq):
                    longest = subseq
                    longest_len = len(longest)
    return longest


def count_subseries(subsequence: List[int], input_list: List[int]):
    subs_i = 0
    count = 0
    for entry in input_list:
        if entry == subsequence[subs_i]:
            subs_i += 1
        else:
            subs_i = 0

        if subs_i >= len(subsequence):
            count += 1
            subs_i = 0
    return count


def remove_repeating_subseqs(subseq: List[int], input_list: List[int]):
    found_first = False
    seq_i = 0
    result = []
    for entry in input_list:
        if subseq[seq_i] == entry:
            seq_i += 1
        else:
            seq_i = 0

        if seq_i == len(subseq):
            if found_first:
                result = result[len(subseq) - 1:]
                seq_i = 0
                continue
            else:
                found_first = True
                seq_i = 0
        result.append(entry)
    return result


if __name__ == '__main__':
    print(
        remove_repeating_subseqs(
            [1, 2, 3], [1, 2, 4, 4, 1, 2, 3, 1, 2, 3, 3, 3, 2, 1, 2, 3]))
