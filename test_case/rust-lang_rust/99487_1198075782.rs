
ws/is_whitespace/issue-38        time:   [527.84 µs 528.70 µs 529.89 µs]
ws/is_ascii/issue-38             time:   [324.65 µs 325.59 µs 326.42 µs]
ws/match/issue-38                time:   [623.96 µs 624.12 µs 624.30 µs]
ws/mapped/issue-38               time:   [468.43 µs 468.51 µs 468.62 µs]
*ws/mapped-if/issue-38            time:   [259.96 µs 260.03 µs 260.11 µs]

ws/is_whitespace/addresses       time:   [45.766 µs 45.776 µs 45.788 µs]
ws/is_ascii/addresses            time:   [38.286 µs 38.305 µs 38.325 µs]
ws/match/addresses               time:   [50.122 µs 50.139 µs 50.159 µs]
ws/mapped/addresses              time:   [36.143 µs 36.153 µs 36.165 µs]
*ws/mapped-if/addresses           time:   [18.072 µs 18.076 µs 18.079 µs]

ws/is_whitespace/first-128       time:   [139.00 ns 139.03 ns 139.05 ns]
ws/is_ascii/first-128            time:   [114.00 ns 114.12 ns 114.27 ns]
ws/match/first-128               time:   [206.35 ns 206.40 ns 206.45 ns]
ws/mapped/first-128              time:   [159.31 ns 159.34 ns 159.38 ns]
*ws/mapped-if/first-128           time:   [82.443 ns 82.460 ns 82.478 ns]

ws/is_whitespace/first-256       time:   [1.3559 µs 1.3563 µs 1.3566 µs]
ws/is_ascii/first-256            time:   [254.32 ns 254.48 ns 254.67 ns]
ws/match/first-256               time:   [501.59 ns 501.72 ns 501.87 ns]
ws/mapped/first-256              time:   [347.05 ns 347.16 ns 347.28 ns]
*ws/mapped-if/first-256           time:   [269.13 ns 269.19 ns 269.26 ns]

ws/is_whitespace/first-4000      time:   [126.43 µs 126.58 µs 126.72 µs]
ws/is_ascii/first-4000           time:   [20.343 µs 20.369 µs 20.402 µs]
ws/match/first-4000              time:   [39.066 µs 39.079 µs 39.095 µs]
*ws/mapped/first-4000             time:   [24.187 µs 24.194 µs 24.203 µs]
ws/mapped-if/first-4000          time:   [31.897 µs 31.911 µs 31.925 µs]

ws/is_whitespace/30K-blanks      time:   [31.680 µs 31.686 µs 31.692 µs]
ws/is_ascii/30K-blanks           time:   [24.103 µs 24.111 µs 24.120 µs]
*ws/match/30K-blanks              time:   [18.106 µs 18.117 µs 18.130 µs]
ws/mapped/30K-blanks             time:   [36.161 µs 36.175 µs 36.191 µs]
*ws/mapped-if/30K-blanks          time:   [18.084 µs 18.089 µs 18.095 µs]

ws/is_whitespace/single-space    time:   [1.2049 ns 1.2054 ns 1.2062 ns]
ws/is_ascii/single-space         time:   [1.4057 ns 1.4060 ns 1.4064 ns]
*ws/match/single-space            time:   [1.0038 ns 1.0043 ns 1.0049 ns]
ws/mapped/single-space           time:   [2.0073 ns 2.0078 ns 2.0084 ns]
*ws/mapped-if/single-space        time:   [1.0042 ns 1.0046 ns 1.0052 ns]

ws/is_whitespace/single-a        time:   [1.2045 ns 1.2049 ns 1.2053 ns]
ws/is_ascii/single-a             time:   [1.0042 ns 1.0049 ns 1.0058 ns]
ws/match/single-a                time:   [1.8074 ns 1.8078 ns 1.8083 ns]
ws/mapped/single-a               time:   [2.0078 ns 2.0082 ns 2.0085 ns]
*ws/mapped-if/single-a            time:   [1.0040 ns 1.0042 ns 1.0043 ns]

ws/is_whitespace/single-tab      time:   [1.2049 ns 1.2055 ns 1.2062 ns]
ws/is_ascii/single-tab           time:   [1.4057 ns 1.4061 ns 1.4065 ns]
*ws/match/single-tab              time:   [806.82 ps 809.74 ps 813.16 ps]
ws/mapped/single-tab             time:   [2.0080 ns 2.0084 ns 2.0088 ns]
ws/mapped-if/single-tab          time:   [1.0042 ns 1.0047 ns 1.0053 ns]

