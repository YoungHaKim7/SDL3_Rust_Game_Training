#include <iostream>
#include <opencv2/opencv.hpp>

using namespace std;
using namespace cv;

// Function to convert the video into a contour frame
cv::Mat contour_frame(cv::Mat frame) {
    // Initialize the matrix to store the contour frame
    cv::Mat gray;
    cv::cvtColor(frame, gray, cv::COLOR_BGR2GRAY);   // Convert the frame to grayscale
    cv::Mat blur;                                   
    cv::GaussianBlur(gray, blur, cv::Size(5, 5), 0); // Apply Gaussian blur to the grayscale frame
    cv::Mat canny;                                    
    cv::Canny(blur, canny, 50, 150);                 // Apply Canny edge detection to the blurred frame
    return canny;
}


int main() {

    // Define the matrix to store video into
    cv::Mat video_from_facecam;
    // Start capturing video
    cv::VideoCapture cap(0);
    // Checking if the video is being captured
    if (!cap.isOpened()) {
        std::cout << "Could not open video stream!" << std::endl;
        return -1;
    }

    while (char (cv::waitKey(1)) != 'q') {
        cap >> video_from_facecam;
        // If video is empty, break the loop
        if (video_from_facecam.empty()) {
            break; 
        }
        
        // Show normal video 
        imshow("Video Player", video_from_facecam);

        // Show contour frame
        imshow("Contour Frame", contour_frame(video_from_facecam));

    }

    cap.release();  // Release the buffer memory
    return 0;
}
