struct T
{
	int sh
	int sh1
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

#if COND1
int T1 = #if INLINE_COND C1 #else C2 #endif
#elseif COND2
int T2 = C3
#elseif COND3
int T3
#else
int T4
#endif