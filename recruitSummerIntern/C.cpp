#include <iostream>
#include <stdio.h>

using namespace std;
int main(int argc, char const* argv[])
{
	ios::sync_with_stdio(false);
	int N;
	cin >> N;
	int rgb[3][N];
	int rgbMax[3]={0};
	int ans=2147483647;
	for (int i=0; i<N; i++) {
		cin >> rgb[0][i] >> rgb[1][i] >> rgb[2][i];
	}
	for (int i=0; i<N; i++) {
		for (int j=0; j<N; j++) {
			if (j==i) {
				continue;
			}
			for (int k=0; k<3; k++) {
				if (rgbMax[k] < rgb[k][j]) {
					rgbMax[k] = rgb[k][j];
				}
			}
		}
		if (ans > rgbMax[0]+rgbMax[1]+rgbMax[2]) {
			ans = rgbMax[0]+rgbMax[1]+rgbMax[2];
		}
		for (int k=0; k<3; k++) {
			rgbMax[k] = 0;
		}

	}
	cout << ans << "\n";
	

	return 0;
}
