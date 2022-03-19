// struct s {
// 	int c[4];
// };

// void f(int* x, int* y) {
//     int c;
//     for (c = 0; c < 4; c++) {
//         y[c] = x[c];
//     }
// }

// int main(__attribute__((private(0))) int a, __attribute__((private(1))) int b) { 
//     int x[4] = {a,b,a,b};

//     struct s y; 
//     f(x, y.c);

//     int sum = 0;
//     for (int i = 0; i < 4; i++) {
//         sum += y.c[i];
//     }
//     return sum;
// }

// int main(__attribute__((private(0))) int a, __attribute__((private(1))) int b) { 
//     int x[1][4];
//     x[0][0] = a;
//     x[0][1] = b;
//     x[0][2] = a;
//     x[0][3] = b;

//     int sum = 0;
//     for (int i = 0; i < 4; i++) {
//         sum += x[0][i];
//     }
//     return sum;
// }

// int min_(int * y) {
// 	int res = 0;
// 	for(int i = 0; i < 4; i++){
// 		res += y[i];
// 	}
//     return res;
// }

// int main(__attribute__((private(0))) int a, __attribute__((private(1))) int b) { 
//     int x[2][4];
//     x[0][0] = a;
//     x[0][1] = b;
//     x[0][2] = a;
//     x[0][3] = b;
//     x[1][0] = a;
//     x[1][1] = b;
//     x[1][2] = a;
//     x[1][3] = b;

//     int c[1];
//     c[0] = min_(x[0]); 
//     int res = c[0];	
//     return res;
// }

#define D 2 // Dimension (fix)
#define NA 10 // Number of data points from Party A
#define NB 10 // Number of data points from Party B
#define NC 5 // Number of clusters
#define PRECISION 1
#define LEN (NA+NB)
#define LEN_OUTER 1
#define LEN_INNER (LEN/LEN_OUTER)

// struct InputA {
//     int dataA[D*NA];
// };

// struct InputB {
//     int dataB[D*NA];
// };

// struct Output {
//     int cluster[D*NC];
// };

int main(__attribute__((private(0))) int a[10], __attribute__((private(1))) int b[10]) { 
    // struct InputA input_a;
    // struct InputB input_b;
    // struct Output output; 

    // for(int c = 0; c < NC; c++) {  
	// 	output.cluster[c*D] = a[0];
	// 	output.cluster[c*D+1] = b[0];
	// }

    // int cc = 1;
    // output.cluster[0] += a[cc + 0];
    // output.cluster[1] += a[cc + 1];
    // output.cluster[2] += a[cc + 2];
    // output.cluster[3] += a[cc + 3];
    // output.cluster[4] += a[cc + 4];
    // output.cluster[5] += a[cc + 5];
    // output.cluster[6] += a[cc + 6];
    // output.cluster[7] += a[cc + 7];
    // output.cluster[8] += a[cc + 8];
    // output.cluster[9] += a[cc + 9];
    
	// return output.cluster[0];
    int cluster[D*NC];

    for(int c = 0; c < NC; c++) {  
		cluster[c*D] = a[0];
		cluster[c*D+1] = b[0];
	}

    int cc = 1;
    cluster[0] += a[cc + 0];
    cluster[1] += a[cc + 1];
    cluster[2] += a[cc + 2];
    cluster[3] += a[cc + 3];
    cluster[4] += a[cc + 4];
    cluster[5] += a[cc + 5];
    cluster[6] += a[cc + 6];
    cluster[7] += a[cc + 7];
    cluster[8] += a[cc + 8];
    cluster[9] += a[cc + 9];
    
	return cluster[0];
}