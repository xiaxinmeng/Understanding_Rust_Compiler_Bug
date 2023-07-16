txt
# case 1: generate profile data

beta --> stage0 --> stage1 =run=> profdata (for uploading)
                    ^^^^^^
               dist (instrumented)


# case 2: use profile data (downloaded, from some previous build)

beta --> stage0 --> stage1
                    ^^^^^^
               dist (profile-use)
