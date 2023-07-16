
        for index in range(0, length):
            cs.append((str(index), (gdb_ptr + index).dereference()))
        return cs
