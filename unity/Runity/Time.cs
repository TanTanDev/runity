using System;
using System.Runtime.InteropServices;
namespace Runity {
    [StructLayout(LayoutKind.Sequential)]
    public struct Time {
        public float delta_time;
    }
}