#include <iostream>
#include <stdio.h>

using namespace std;

int main(int argc, char const* argv[])
{
	ios::sync_with_stdio(false);
	int N, M;
	cin >> N >> M;
	int x;
	char c;
	char ans[N];
	for (int i=0; i<M; i++) {
		cin >> x >> c;
		ans[x] = c;
		ans[N+1-x] = c;
	}
	for(int i = 0; i < N+1; i++){
		printf("%c", ans[i]);
	}
	cout << "\n";

	return 0;
}
