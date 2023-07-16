
class AddAssign rhs self where
    add_assign :: IORef self -> rhs -> IO () -- signature of method
    default add_assign :: Add rhs self self => IORef self -> rhs -> IO () -- signature for default
    add_assign self rhs = do
        self' <- readIORef self
        writeIORef self (add self' rhs)
