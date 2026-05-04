#include <stdio.h>

int square(const int *n)
{
    return (*n) * (*n);
}

int main()
{
    int n = 1;
    int square_result;

    for (long i=0; i<3000000000; i++)
    {
        square_result = square(&n);
        n += square_result;
    }
    printf("Value: %d\n", n);
}
