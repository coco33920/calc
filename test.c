#include <math.h>
#include <stdio.h>
#include <stdlib.h>
/* INPUT: A - array of pointers to rows of a square matrix having dimension N
 *        Tol - small tolerance number to detect failure when the matrix is near degenerate
 * OUTPUT: Matrix A is changed, it contains a copy of both matrices L-E and U as A=(L-E)+U such that P*A=L*U.
 *        The permutation matrix is not stored as a matrix, but in an integer vector P of size N+1 
 *        containing column indexes where the permutation matrix has "1". The last element P[N]=S+N, 
 *        where S is the number of row exchanges needed for determinant computation, det(P)=(-1)^S    
 */
int LUPDecompose(double **A, int N, double Tol, int *P) {

    int i, j, k, imax; 
    double maxA, *ptr, absA;

    for (i = 0; i <= N; i++)
        P[i] = i; //Unit permutation matrix, P[N] initialized with N

    for (i = 0; i < N; i++) {
        maxA = 0.0;
        imax = i;

        for (k = i; k < N; k++)
            if ((absA = fabs(A[k][i])) > maxA) { 
                maxA = absA;
                imax = k;
            }

        if (maxA < Tol) return 0; //failure, matrix is degenerate

        if (imax != i) {
            //pivoting P
            j = P[i];
            P[i] = P[imax];
            P[imax] = j;

            //pivoting rows of A
            ptr = A[i];
            A[i] = A[imax];
            A[imax] = ptr;

            //counting pivots starting from N (for determinant)
            P[N]++;
        }
	

	
	for(int si = 0; si < N; si++){
		for(int sj = 0; sj < N; sj++){
		printf("a[%d][%d] = %f\n",si,sj,A[si][sj]);
		}
	}
	printf("\n\n");

        for (j = i + 1; j < N; j++) {
            A[j][i] /= A[i][i];

            for (k = i + 1; k < N; k++)
                A[j][k] -= A[j][i] * A[i][k];
        }
    }

    return 1;  //decomposition done 
}

/* INPUT: A,P filled in LUPDecompose; b - rhs vector; N - dimension
 * OUTPUT: x - solution vector of A*x=b
 */
void LUPSolve(double **A, int *P, double *b, int N, double *x) {

    for (int i = 0; i < N; i++) {
        x[i] = b[P[i]];

        for (int k = 0; k < i; k++)
            x[i] -= A[i][k] * x[k];
    }

    for (int i = N - 1; i >= 0; i--) {
        for (int k = i + 1; k < N; k++)
            x[i] -= A[i][k] * x[k];

        x[i] /= A[i][i];
    }
}

/* INPUT: A,P filled in LUPDecompose; N - dimension
 * OUTPUT: IA is the inverse of the initial matrix
 */
void LUPInvert(double **A, int *P, int N, double **IA) {
  
    for (int j = 0; j < N; j++) {
        for (int i = 0; i < N; i++) {
            IA[i][j] = P[i] == j ? 1.0 : 0.0;

            for (int k = 0; k < i; k++)
                IA[i][j] -= A[i][k] * IA[k][j];
        }

        for (int i = N - 1; i >= 0; i--) {
            for (int k = i + 1; k < N; k++)
                IA[i][j] -= A[i][k] * IA[k][j];

            IA[i][j] /= A[i][i];
        }
    }
}

/* INPUT: A,P filled in LUPDecompose; N - dimension. 
 * OUTPUT: Function returns the determinant of the initial matrix
 */
double LUPDeterminant(double **A, int *P, int N) {

    double det = A[0][0];

    for (int i = 1; i < N; i++)
        det *= A[i][i];

    return (P[N] - N) % 2 == 0 ? det : -det;
}

int main() {

 int N = 4;
 double *A1 = malloc(N*sizeof(double));
 double *A2 = malloc(N*sizeof(double));
 double *A3 = malloc(N*sizeof(double));
 double *A4 = malloc(N*sizeof(double));
 double **A = malloc(N*sizeof(A1));
 int *P = malloc((N+1)*sizeof(int));
 //1,2,3,4;5,0,6,0;7,0,9,10;11,0,12,13
A1[0] = 1;
A1[1] = 2;
A1[2] = 3;
A1[3] = 4;
A2[0] = 5;
A2[1] = 0;
A2[2] = 6;
A2[3] = 0;
A3[0] = 7;
A3[1] = 0;
A3[2] = 9;
A3[3] = 10;
A4[0] = 11;
A4[1] = 0;
A4[2] = 12;
A4[3] = 13;
A[0] = A1;
A[1] = A2;
A[2] = A3;
A[3] = A4;
for(int i = 0; i < N+1; i++){
	P[i] = i;
}
LUPDecompose(A, N, 1e-10, P);


for(int i = 0; i < N; i++){
	for(int j = 0; j < N; j++){
		printf("a[%d][%d] = %f\n",i,j,A[i][j]);
	}
}

double det = LUPDeterminant(A, P, N);
printf("det: %f\n",det);

return 0;

}
