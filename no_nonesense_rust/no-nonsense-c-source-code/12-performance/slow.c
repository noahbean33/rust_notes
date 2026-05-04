#include <stdio.h>

int square(int n)
{
    return n * n;
}

int main()
{
    int n = 1;
    for (long i=0; i<3000000000; i++)
    {
        int square_result = square(n);
        n += square_result;
    }
    printf("Value: %d\n", n);
}
