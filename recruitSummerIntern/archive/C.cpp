#include <iostream>
using namespace std;

int main(int argc, char const* argv[])
{
	int M, N, ans=0;
	cin >> M >> N;
	for (int i=0; i<M; i++) {
		for (int j=0; j<N; j++) {
			for (int k=0; k<3; k++) {
				ans++;
			}
		}
	}
	cout << ans << "\n";
	return 0;
}

/*
 * 1 -> 1,2,3
 * 2, 1 -> 0
 * 3, 1 -> 1,2,1  1,3,1  1,2,3  1,3,2 2…
 * */
