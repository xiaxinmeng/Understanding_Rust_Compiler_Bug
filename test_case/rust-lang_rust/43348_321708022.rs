c
typedef struct WSAData {
    WORD          wVersion;
    WORD          wHighVersion;
#ifdef _WIN64
    unsigned short     iMaxSockets;
    unsigned short     iMaxUdpDg;
    char FAR *       lpVendorInfo;
    char          szDescription[WSADESCRIPTION_LEN+1];
    char          szSystemStatus[WSASYS_STATUS_LEN+1];
#else
    char          szDescription[WSADESCRIPTION_LEN+1];
    char          szSystemStatus[WSASYS_STATUS_LEN+1];
    unsigned short     iMaxSockets;
    unsigned short     iMaxUdpDg;
    char FAR *       lpVendorInfo;
#endif
} WSADATA, FAR * LPWSADATA;
