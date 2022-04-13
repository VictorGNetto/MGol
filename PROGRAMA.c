#include <stdio.h>

typedef char literal[256];
typedef int inteiro;
typedef double real;

void main(void)
{
    /*----Variaveis temporarias----*/
    real T0
    inteiro T1
    inteiro T2
    inteiro T3
    /*------------------------------*/
    literal A;
    inteiro B;
    inteiro D;
    real C;

    T0 = C + 2e0;
    C = T0;
    printf("Digite A:");
    scanf("%s", A);
    T1 = B > 2;
    if (T1) {
    scanf("%d", &B);
    T2 = B <= D;
    if (T2) {
    printf("B esta entre 2 e 4");
    }
    }
    T3 = C < 5;
    for(; T3; T3 = C < 5) {
    printf("%lf", C);
    scanf("%lf", &C);
    }
    return 0;
}
