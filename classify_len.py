#coding：utf-8
import os,re
import traceback
 

 
def analyze_code(codefilesource):
    '''
    打开一个py文件，统计其中的代码行数，包括空行和注释
    返回含该文件总行数，注释行数，空行数的列表
    :param codefilesource:
    :return:
    '''
    total_line = 0
    comment_line = 0
    blank_line = 0
    with open(codefilesource,'r') as f:
        lines = f.readlines()
        total_line = len(lines)
        line_index = 0


        while line_index < total_line:
            line = lines[line_index]

            if line.startswith("#"):
                comment_line += 1
            elif "import" in line:
                comment_line += 1
            elif re.match("\s*'''",line) is not None:
                comment_line += 1
            elif re.match('\s*"""',line) is not None:
                comment_line += 1
                # while re.match(".*'''$",line) is None:
                #     line = lines[line_index]
                #     comment_line += 1
                #     line_index += 1

            elif line =='\n':
                blank_line += 1
            line_index += 1

    # print("在%s中:"%codefilesource)
    # print("代码行数：",total_line)
    # print("注释行数:",comment_line)
    # print("空行数:", blank_line)
    return [total_line,comment_line,blank_line]




import ast


def getCTX(anode):
    ctx = ""
    if isinstance(anode,ast.Store):
        ctx = "Store"
    if isinstance(anode,ast.Load):
        ctx = "Load"
    if isinstance(anode,ast.Del):    
        # ctx = "Del"
        ctx = "Load"
    return ctx



class CellVisitor(ast.NodeVisitor):
    def __init__(self):
        self.funcRange = {}
        self.funcargs = {}
        self.classRange = {}
        self.callname =set()
        self.calldic = {}
        self.attrname = set()
        self.allname = set()
        self.callarg = set()
        self.loopvar = {}


    def visit_FunctionDef(self, node):
        self.generic_visit(node)
        funcname = node.name 
        startPos = node.lineno
        endPos = node.end_lineno
        argsNum = 0
        if node.args.args:
            argsNum = len(node.args.args)
            arglist=[]
            for arg in node.args.args:
                arglist.append(arg.arg)

            self.funcargs[(funcname,node.lineno,node.col_offset,argsNum,"function")] = arglist



        self.funcRange[(funcname,node.lineno,node.col_offset,argsNum,"function")] = [startPos,endPos]


    def visit_ClassDef(self, node):
        self.generic_visit(node)
        classname = node.name 
        startPos = node.lineno
        endPos = node.end_lineno
        baseNum = len(node.bases) 

        self.classRange[(classname,node.lineno,node.col_offset,baseNum,"class")] = [startPos,endPos]



    def visit_Attribute(self,node):
        self.generic_visit(node)
        # print(ast.dump(node))
        if isinstance(node.value,ast.Name):
            self.attrname.add((node.value.id,node.lineno,node.col_offset,getCTX(node.ctx)))
            # print(node.value.id,node.lineno,node.col_offset)


    def visit_Name(self, node):
        self.generic_visit(node)

        self.allname.add((node.id,node.lineno,node.col_offset,getCTX(node.ctx)))



    def visit_Call(self, node):
        self.generic_visit(node)
        # print(ast.dump(node))
        if isinstance(node.func, ast.Attribute):
            pass
        else:
            # print(dir(node.func))
            self.callname.add((node.func.id,node.lineno,node.col_offset,getCTX(node.func.ctx)))


            arglist = []
            if node.args:
                for item in node.args:
                    if isinstance(item, ast.Name):
                        self.callarg.add((item.id,item.lineno,item.col_offset,getCTX(item.ctx)))
                        arglist.append((item.id,item.lineno,item.col_offset,getCTX(item.ctx)))


            self.calldic[(node.func.id,node.lineno,node.col_offset,getCTX(node.func.ctx))] = arglist






def average(infodic,key):
    c = 0
    s = 0
    for info in infodic:
        if key in infodic[info]:
            s =s + infodic[info][key]
            c = c + 1
    ave = s/c
    return ave

 
if __name__ == '__main__':
    
    dirpath = '/home/xxm/Desktop/empirical_type_checker/dataset/project/mypy'
    # dirpath='/home/xxm/Desktop/cross-testing/dataset_bkp'
    destdir = {}
    c = 0
    maxloc = []
    infodic = {} 
    loaddic = {}
    storedic = {}
    codeline={}
    exceptlist = []
    for root,dirs,files in os.walk(dirpath):
        for file in files:
            if file.endswith('.py') and file != "temp.py": 
                inputfile = root + "/" +file
                try:
                    c = c + 1

                    f= open(inputfile,'r').read()


                    myast = ast.parse(f)
                    astvisitor = CellVisitor()
                    astvisitor.visit(myast)
                    funcargs = astvisitor.funcargs


                    classname = astvisitor.classRange
                    funcname =astvisitor.funcRange
                    callname = astvisitor.callname
                    calldic = astvisitor.calldic
                    loopvar = astvisitor.loopvar
                    # print(astvisitor.allname)
                    var = astvisitor.allname - astvisitor.attrname - callname

                    loadnum = 0
                    storenum = 0
                    for item in var:
                        if item[3] == 'Load':
                            loadnum =loadnum +1
                        else:
                            storenum =storenum +1

                    varlen =len(var)



                    vlen = 3000000000

                    if varlen <= vlen:
                        if str(loadnum) in loaddic.keys():
                            loaddic[str(loadnum)]=loaddic[str(loadnum)]+1
                        else:
                            loaddic[str(loadnum)]=1 

                    if varlen <= vlen:
                        if str(storenum) in storedic.keys():
                            storedic[str(storenum)]=storedic[str(storenum)]+1
                        else:
                            storedic[str(storenum)]=1 

                    lines = analyze_code(inputfile)
                    line = lines[0]
                    if varlen <= vlen:
                        if str(line) in codeline.keys():
                            codeline[str(line)]=codeline[str(line)]+1
                        else:
                            codeline[str(line)]=1 

                    # print(var)
                    if varlen <= vlen:
                        if str(varlen) in destdir.keys():
                            destdir[str(varlen)]=destdir[str(varlen)]+1
                        else:
                            destdir[str(varlen)]=1    
                    # else:

                    #     os.remove(inputfile)
                    #     print(str(varlen),inputfile)
                        # if 'max' in destdir.keys():
                        #     destdir['max'] = destdir['max']+1
                        # else:
                        #     destdir['max']=0


                # print()
                except:
                    # print(traceback.print_exc())
                    pass

    s = 0
    print("ssssssssssssss")
    sorted(destdir.keys(),reverse=True)
    for item in destdir:
        print("total",item, destdir[item])
        # print("(",item,",", destdir[item],"),")
        s = s+destdir[item]
    print(s,'\n')


    s = 0
    for item in loaddic:
        print("load",item, loaddic[item])
        # print("(",item,",", loaddic[item],"),")
        s = s+loaddic[item]
    print(s,'\n')


    s = 0
    for item in storedic:
        print("store",item, storedic[item])
        # print("(",item,",",storedic[item],"),")
        s = s+storedic[item]
    print(s,'\n')

    s = 0
    for item in codeline:
        # print("line",item, codeline[item])
        # print("(",item,",", codeline[item],"),")
        s = s+codeline[item]
    print(s,'\n')
