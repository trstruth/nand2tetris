for idx in range(16):
    procedure_string = f'''
    And(a=s0, b=a[{idx}], out=aout{idx});
    And(a=s1, b=b[{idx}], out=bout{idx});
    And(a=s2, b=c[{idx}], out=cout{idx});
    And(a=s3, b=d[{idx}], out=dout{idx});
    Or(a=aout{idx}, b=bout{idx}, out=i0{idx});
    Or(a=i0{idx}, b=cout{idx}, out=i1{idx});
    Or(a=i1{idx}, b=dout{idx}, out=out[{idx}]);'''

    print(procedure_string)