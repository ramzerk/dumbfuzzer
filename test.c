#include <stdio.h>
#include <string.h>

int main (int ac, char **av){

    char passwd[9] = "Fpnxweoe\0";

    if (ac == 1 || ac > 2 )
    {
        printf("Enter the password\n");
        return 1;
    }

    else if  (strlen(av[1]) > 8)
    {
        printf("Password Err\n");
        return 1;
    }
    if (!strcmp(passwd, av[1]))
    {
        printf("Success\n");
        return 0;
    }
    else
        printf("Wrong Password\n");
    return 1;

}
