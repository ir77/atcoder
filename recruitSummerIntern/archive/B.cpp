#include <iostream>
using namespace std;

int main(int argc, char const* argv[])
{
	int N, input, sum=0, ans=0;
	cin >> N;
	for (int i=0; i<N; i++) {
		for (int j=0; j<5; j++) {
			cin >> input;
			sum += input;
		}
		if (sum < 20) {
			ans++;
		}
		sum = 0;
	}
	cout << ans << "\n";
	return 0;
}
