#include <iostream>
#include <cstdlib>
#include <typeinfo>
#include <string>
using namespace std;

int main(int argc, char const* argv[])
{
	int B, L, N;
	string S, ans="", stackStr="";
	bool flag=false;

	cin >> B >> L >> N;
	cin >> S;

	for (int i=0; i<N; i++) {
		if (S[i] == '(' ) {
			flag = true;
			continue;
		}
		else if (S[i] == ')' ) {
			flag = false;
			i++;
			for (int j=0; j<atoi(S[i]); j++) {
				ans += stackStr;
			}
			stackStr = "";
		}

		if (!flag) {
			ans += S[i];
		} else {
			stackStr+=S[i];
		}
	}
	cout << ans << "\n";

	return 0;
}
