
import subprocess

def main():
    proc = subprocess.Popen(["python",  "-c", "print('a' * 10000000)"],
                            shell=False, stdout=subprocess.PIPE)
    proc.communicate()
    print "Calling wait()"
    proc.wait()
    print "wait() returned"

if __name__ == '__main__':
    main()
