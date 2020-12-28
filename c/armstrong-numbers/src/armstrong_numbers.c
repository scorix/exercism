#include "armstrong_numbers.h"

bool is_armstrong_number(int candidate) {
    int digits = ceil(log10(candidate));
    digits += candidate == pow(10, digits) ? 1 : 0;

    int res = 0;
    int tmp_candidate = candidate;

    for (int i = 0; i < digits; i++)
    {
        int x = tmp_candidate % 10;
        res += pow(x, digits);
        tmp_candidate /= 10;
    }
    
    return res == candidate;
}
