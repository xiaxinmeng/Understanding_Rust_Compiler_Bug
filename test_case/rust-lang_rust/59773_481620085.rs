
 public static void main(String[] args) {
        Date start=new Date();
        ArrayList<String> list=new ArrayList<String>();
        list.add("dsf");
        list.add("fsadf");
        for (int i=0;i<100000;i++){
            list.clone();
        }
       System.out.println((new Date().getTime()-d.getTime())/100000f+" ns/op");
    }
//BenchmarkClone            150ns/op
