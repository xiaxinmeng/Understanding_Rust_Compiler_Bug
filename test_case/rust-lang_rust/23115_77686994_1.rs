
        let lo = (span.lo + imported_filemaps[filemap_index].original_start_pos) -
                    imported_filemaps[filemap_index].translated_filemap.start_pos;
        let hi = (span.hi + imported_filemaps[filemap_index].original_start_pos) -
                    imported_filemaps[filemap_index].translated_filemap.start_pos;
