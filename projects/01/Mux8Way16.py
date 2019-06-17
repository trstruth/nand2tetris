for idx in range(16):
    procedure_string = f'''
    And(a=s000, b=a[{idx}], out=aout{idx});
    And(a=s001, b=b[{idx}], out=bout{idx});
    And(a=s010, b=c[{idx}], out=cout{idx});
    And(a=s011, b=d[{idx}], out=dout{idx});
    And(a=s100, b=e[{idx}], out=eout{idx});
    And(a=s101, b=f[{idx}], out=fout{idx});
    And(a=s110, b=g[{idx}], out=gout{idx});
    And(a=s111, b=h[{idx}], out=hout{idx});
    Or(a=aout{idx}, b=bout{idx}, out=i0{idx});
    Or(a=i0{idx}, b=cout{idx}, out=i1{idx});
    Or(a=i1{idx}, b=dout{idx}, out=i2{idx});
    Or(a=i2{idx}, b=eout{idx}, out=i3{idx});
    Or(a=i3{idx}, b=fout{idx}, out=i4{idx});
    Or(a=i4{idx}, b=gout{idx}, out=i5{idx});
    Or(a=i5{idx}, b=hout{idx}, out=out[{idx}]);'''

    print(procedure_string)