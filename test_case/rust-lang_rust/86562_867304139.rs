rs
        for g in groups // groups: IntoIterator<SomeType> where SomeType: Index<usize>
        {
            if same_row_group(&state, &sr, &br, row, g[0]) // g: Index<usize>
            {
                g.push(row);
                fits_existing_group = true;
                break;
            }
        }
