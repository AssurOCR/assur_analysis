#include <tesseract/baseapi.h>
#include <leptonica/allheaders.h>
#include <opencv2/opencv.hpp>
#include <opencv2/ml.hpp>
#include <iostream>


struct CharacterInfo {
    int x1;
    int y1;
    int x2;
    int y2;
    char *character;
    float conf;
};

struct Page {
    char * location;
    char * data;
    int n_characters;
    CharacterInfo * characters;
};

struct Page recognize_page(tesseract::TessBaseAPI * api, char * page_location) {
    // Open input image with leptonica library
    Pix * image = pixRead(page_location);
    api -> SetImage(image);
    // Get OCR result
    char * outText = api -> GetUTF8Text();
    //printf("OCR output:\n%s", outText);

    Page page;
    page.location = page_location;
    page.data = outText;

    tesseract::ResultIterator * ri = api -> GetIterator();
    tesseract::PageIteratorLevel level = tesseract::RIL_SYMBOL;

    int n_characters = 0;

    if (ri != 0) {
        do {
            int x1, y1, x2, y2;

            ri -> BoundingBox(level, & x1, & y1, & x2, & y2);
            n_characters++;
        } while (ri -> Next(level));
    }

    page.n_characters = n_characters;
    page.characters = (CharacterInfo * ) malloc(page.n_characters * sizeof(CharacterInfo));

    int i = 0;


    ri = api -> GetIterator();
    if (ri != 0) {
        do {
            int x1, y1, x2, y2;
            ri -> BoundingBox(level, & x1, & y1, & x2, & y2);

            page.characters[i].x1 = x1;
            page.characters[i].x2 = x2;
            page.characters[i].y1 = y1;
            page.characters[i].y2 = y2;
            page.characters[i].conf = ri -> Confidence(level);

            page.characters[i].character =  ri->GetUTF8Text(level);

            //printf("%s", ri->GetUTF8Text(level));

            i++;
        } while (ri -> Next(level));
    }

    delete ri;

    pixDestroy( & image);

    return page;
}

extern "C" {
struct Page * ocr_pages(char ** pages, int n_pages, char* langs) {
    tesseract::TessBaseAPI * api = new tesseract::TessBaseAPI();
    // Initialize tesseract-ocr with English, specifying tessdata path
    if (api -> Init("./tessdata", langs)) {
        fprintf(stderr, "Could not initialize tesseract.\n");
        exit(1);
    }

    // Allocate an array of Page structs
    struct Page * recognized_pages = (struct Page * ) malloc(n_pages * sizeof(struct Page));

    // Iterate over the array and add the recognized text to the array
    for (int i = 0; i < n_pages; i++) {
        recognized_pages[i] = recognize_page(api, pages[i]);
    }

    // Destroy used object and release memory
    api -> End();
    delete api;

    return recognized_pages;
}
}

extern "C" {
//free all page data
void free_pages(Page* pages, int total_page) {
    for (int i = 0; i < total_page; i++) {
        free(pages[i].characters);
    }
    free(pages);

    //printf("I freed all. I can now retire!\n\n");
}
}


cv::Mat denoise(cv::Mat& img) {
    cv::Mat denoised_img;
    cv::fastNlMeansDenoising(img, denoised_img, 10, 7, 21);

    return denoised_img;
}


extern "C" {
    double compute_skew(const cv::Mat& image) {
    // Binarize the image using Otsu's method
    cv::Mat binary;
    cv::threshold(image, binary, 0, 255, cv::THRESH_BINARY | cv::THRESH_OTSU);

    // Find the contours of the connected components in the binary image
    std::vector<std::vector<cv::Point> > contours;
    cv::findContours(binary, contours, cv::RETR_EXTERNAL, cv::CHAIN_APPROX_SIMPLE);

    // Select the contour with the largest area
    int max_area = 0;
    int max_index = 0;
    for (size_t i = 0; i < contours.size(); i++) {
        int area = cv::contourArea(contours[i]);
        if (area > max_area) {
        max_area = area;
        max_index = i;
        }
    }

    // Compute the minimum area bounding rectangle of the selected contour
    cv::RotatedRect rect = cv::minAreaRect(contours[max_index]);

    // Extract the rotation angle from the bounding rectangle
    double angle = rect.angle;
    if (rect.size.width < rect.size.height) {
        angle += 90.0;
    }

    return angle;
    }

    double deskew(const std::string& image_file, const std::string& dest_file) {
    // Load the image
    cv::Mat image = cv::imread(image_file, cv::IMREAD_GRAYSCALE);
    //image = denoise(image);

    if (image.empty()) {
        std::cerr << "Failed to load image: " << image_file << std::endl;
        return 0.0;
    }

    // Calculate the rotation angle
    double angle = compute_skew(image);

    // Deskew the image
    cv::Mat deskewed;
    cv::Point2f center(image.cols / 2.0, image.rows / 2.0);
    cv::Mat rot_mat = cv::getRotationMatrix2D(center, angle, 1.0);
    cv::warpAffine(image, deskewed, rot_mat, image.size(), cv::INTER_LINEAR, cv::BORDER_CONSTANT, cv::Scalar(255, 255, 255));

    // Save the deskewed image
    cv::imwrite(dest_file, deskewed);

    return angle;
    }

}
