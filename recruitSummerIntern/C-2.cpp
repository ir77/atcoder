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
		for (int j=0; j<3; j++) {
			if (rgbMax[j] < rgb[j][i]) {
				rgbMax[j] = rgb[j][i];
			}
		}
		
	}
	for (int i=0; i<N; i++) {
		int tmprgbMax[3]={0};
		for (int j=0; j<3; j++) {
			if (rgb[j][i] == rgbMax[j]) {
				// search second
				for (int k=0; k<N; k++) {
					if (i==k) {
						continue;
					}
					if (tmprgbMax[j] < rgb[j][k]) {
						tmprgbMax[j] = rgb[j][k];
					}
				}
			} else {
				tmprgbMax[j] = rgbMax[j];
			}
		}

		if (ans > tmprgbMax[0]+tmprgbMax[1]+tmprgbMax[2]) {
			ans = tmprgbMax[0]+tmprgbMax[1]+tmprgbMax[2];
		}
	}
	cout << ans << "\n";
	

	return 0;
}
