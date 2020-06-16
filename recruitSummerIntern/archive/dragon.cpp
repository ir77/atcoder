#include <iostream>
using namespace std;

int main(int argc, char const* argv[])
{
	unsigned long long member = 42;
	for (;;) {
		member = member + member;
		if (member > 130000000) {
			break;
		}
	}
	cout << member << "\n";
	return 0;
}
