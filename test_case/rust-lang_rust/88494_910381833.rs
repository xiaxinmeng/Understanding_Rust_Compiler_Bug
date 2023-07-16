cs
public class LingerOption {
   public bool Enabled { get; set; }
   public int LingerTime { get; set; }
}

public class TcpClient {
    // ...
    public LingerOption? LingerState { get; set; }
}
