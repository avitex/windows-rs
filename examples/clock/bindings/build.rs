fn main() {
    windows::build! {
        Windows::{
            Foundation::Numerics::Matrix3x2,
            Win32::{
                Foundation::{DXGI_STATUS_OCCLUDED, HINSTANCE, HWND, LPARAM, LRESULT, PSTR, WPARAM},
                Graphics::{
                    Direct2D::{
                        CLSID_D2D1Shadow, D2D1CreateFactory, ID2D1Bitmap1, ID2D1Device,
                        ID2D1DeviceContext, ID2D1Effect, ID2D1Factory1, ID2D1SolidColorBrush,
                        ID2D1StrokeStyle, D2D1_BITMAP_PROPERTIES1, D2D1_BRUSH_PROPERTIES, D2D1_COLOR_F,
                        D2D1_COMPOSITE_MODE, D2D1_DEVICE_CONTEXT_OPTIONS, D2D1_ELLIPSE,
                        D2D1_FACTORY_OPTIONS, D2D1_INTERPOLATION_MODE, D2D1_PIXEL_FORMAT,
                        D2D1_STROKE_STYLE_PROPERTIES, D2D1_UNIT_MODE, D2D_POINT_2F, D2D_RECT_F,
                        D2D_SIZE_F, D2D_SIZE_U,
                    },
                    Direct3D11::{D3D11CreateDevice, ID3D11Device, D3D11_SDK_VERSION, D3D_DRIVER_TYPE},
                    Dxgi::{
                        CreateDXGIFactory1, IDXGIDevice, IDXGIFactory2, IDXGIFactory7, IDXGIOutput,
                        IDXGISurface, IDXGISwapChain1, DXGI_ERROR_UNSUPPORTED, DXGI_PRESENT_TEST,
                        DXGI_SAMPLE_DESC, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
                        DXGI_USAGE_RENDER_TARGET_OUTPUT,
                    },
                    Gdi::{BeginPaint, EndPaint, PAINTSTRUCT},
                },
                System::{
                    LibraryLoader::GetModuleHandleA,
                    Performance::{QueryPerformanceCounter, QueryPerformanceFrequency},
                    SystemInformation::GetLocalTime,
                },
                UI::{
                    Animation::{
                        IUIAnimationManager, IUIAnimationTransition, IUIAnimationTransitionLibrary,
                        IUIAnimationVariable, UIAnimationManager, UIAnimationTransitionLibrary,
                        UI_ANIMATION_UPDATE_RESULT,
                    },
                    WindowsAndMessaging::{
                        CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, GetWindowLongA,
                        GetWindowLongPtrA, LoadCursorW, PeekMessageA, PostQuitMessage, RegisterClassA,
                        SetWindowLongA, SetWindowLongPtrA, CREATESTRUCTA, CW_USEDEFAULT, IDC_HAND, MSG,
                        SIZE_MINIMIZED, WINDOW_LONG_PTR_INDEX, WM_ACTIVATE, WM_DESTROY,
                        WM_DISPLAYCHANGE, WM_NCCREATE, WM_PAINT, WM_QUIT, WM_SIZE, WM_USER, WNDCLASSA,
                    },
                },
            },
        },
    };
}
