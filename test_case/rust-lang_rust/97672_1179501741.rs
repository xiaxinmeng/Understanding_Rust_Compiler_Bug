
struct addr_info hints, *list;
... code fills in hints  using hints.ai_family = AF_UNSPEC ...
getaddrinfo(0, "http", &hints, &list);
itn s = socket(0, list[0].ai_family, list[0].ai_socktype, list[0].ai_protocol); 
// normally you run through the list
// until you get a connection using list[0].ai_next
bind(s, list[0].ai_arrd, lis[0].ai_addrlen);
