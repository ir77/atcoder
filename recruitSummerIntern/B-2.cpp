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
	string str="";
	for (int i=0; i<N; i++) {
		str += "a";
	}

	for (int j=0; j<M; j++) {
		cin >> x >> c;
		str[x-1] = c;
		str[N+1-x-1] = c;
	}
	cout << str << "\n";

	return 0;
}
