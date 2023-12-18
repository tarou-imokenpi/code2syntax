# 予約語の使用例
"""
aadadadad
a
da
d
ad
a
"""
# データ型
int_var = 10
float_var = 3.14
str_var = "Hello, World!"
bool_var = True
True
# 制御構造
if bool_var:
    print(str_var)
else:
    print("Not executed")
if str_var == "Hello, World!":
    print(str_var)
else:
    print("Not executed")

# ループ
for i in range(int_var):
    print(i)


# 関数定義
def sample_function(param1, param2):
    result = param1
    return result


# クラス定義
class SampleClass:
    """Sample Class

    Attributes:
        name (str): インスタンス名
    """

    def __init__(self, name):
        self.name = name

    def get_name(self):
        return self.name

    class SampleClass:
        """
        adadaddadad
        """

        def __init__(self, name):
            self.name = name

        def get_name(self):
            return self.name


# インスタンス化
obj = SampleClass("Sample Instance")

# メソッド呼び出し
print(obj.get_name())
