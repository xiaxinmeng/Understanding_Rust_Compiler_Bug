
                return Err(WinError::last(format!("WaitForMultipleObjectsEx returned {}", index)));
