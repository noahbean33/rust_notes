#include <stdio.h>

/* Intentionally broken: attempts to "increment" a value */
void increment(int value)
{
    value = value + 1;
}

int main(void)
{
    int counter;

    increment(counter);

    printf("Counter value: %s\n", counter);

    return 0;
}
