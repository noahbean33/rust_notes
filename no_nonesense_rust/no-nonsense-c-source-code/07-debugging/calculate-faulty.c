#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[])
{
    int sum;
    for (int i=0; i<argc; i++)
    {
        sum = sum + argv[i];
    }

    printf("The sum is %d\n", sum);
}
