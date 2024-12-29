# Result

```bash
g++ src/main.cpp -o app  \
          -I /usr/include/opencv2 \
          -L /usr/lib \
          -lopencv_core \
          -lopencv_imgproc
/usr/bin/ld: /tmp/cc5x734M.o: in function `contour_frame(cv::Mat)':
main.cpp:(.text+0x90): undefined reference to `cv::cvtColor(cv::_InputArray const&, cv::_OutputArray const&, int, int, cv::AlgorithmHint)'
/usr/bin/ld: main.cpp:(.text+0x13a): undefined reference to `cv::GaussianBlur(cv::_InputArray const&, cv::_OutputArray const&, cv::Size_<int>, double, double, int, cv::AlgorithmHint)'
/usr/bin/ld: /tmp/cc5x734M.o: in function `main':
main.cpp:(.text+0x336): undefined reference to `cv::VideoCapture::VideoCapture(int, int)'
/usr/bin/ld: main.cpp:(.text+0x345): undefined reference to `cv::VideoCapture::isOpened() const'
/usr/bin/ld: main.cpp:(.text+0x39e): undefined reference to `cv::VideoCapture::operator>>(cv::Mat&)'
/usr/bin/ld: main.cpp:(.text+0x411): undefined reference to `cv::imshow(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&, cv::_InputArray const&)'
/usr/bin/ld: main.cpp:(.text+0x4ca): undefined reference to `cv::imshow(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&, cv::_InputArray const&)'
/usr/bin/ld: main.cpp:(.text+0x51d): undefined reference to `cv::waitKey(int)'
/usr/bin/ld: main.cpp:(.text+0x53c): undefined reference to `cv::VideoCapture::release()'
/usr/bin/ld: main.cpp:(.text+0x550): undefined reference to `cv::VideoCapture::~VideoCapture()'
/usr/bin/ld: main.cpp:(.text+0x630): undefined reference to `cv::VideoCapture::~VideoCapture()'
collect2: error: ld returned 1 exit status
```
