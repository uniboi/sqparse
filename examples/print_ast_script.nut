struct T
{
	int sh
	#ifCLIENT
	#if 2
	int cl
	int cl2
	#endif
	#endif
}

#if 1
#if 2
table<int, table<int>> t

#endif
#endif
