#include <iostream>
using namespace std;

bool checker(int A, int B, int X) {
	for (int i=A; i<=B-2; i++) {
		for (int j=A+1; j<=B-1; j++) {
			for (int k=A+2; k<=B; k++) {
				if (i+j+k == X) {
					return true;
				}
			}
		}
	}
	return false;
}

int main(int argc, char const* argv[])
{
	ios::sync_with_stdio(false);
	int A, B, X;
	cin >> A >> B >> X;
	if (checker(A, B, X)) {
		cout << "YES" << "\n";
	} else {
		cout << "NO" << "\n";
	}
	return 0;
}
