
A begins unparking (triggered by some third thread)
B unparks A
A completes unparking
B returns from `unpark`
