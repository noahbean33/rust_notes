#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[])
{
    int sum = 0;
    for (int i=1; i<argc; i++)
    {
        char *number_as_string = argv[i];
        int number = atoi(number_as_string);
        sum = sum + number;
    }

    printf("The sum is %d\n", sum);
}
