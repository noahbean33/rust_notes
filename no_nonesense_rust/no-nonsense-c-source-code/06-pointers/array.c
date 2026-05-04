#include <stdio.h>

void modify_array(int array[])
{
    array[0] = 7;
    array[1] = 7;
    array[2] = 7;
}

void print_array(int array[])
{
    printf("array[0] = %d\n", array[0]);
    printf("array[1] = %d\n", array[1]);
    printf("array[2] = %d\n", array[2]);
}

int main()
{
    printf("Creating an array ...\n");
    int array[3] = {0, 4, 9};
    print_array(array);

    printf("Modifying array ...\n");
    modify_array(array);
    print_array(array);
}
