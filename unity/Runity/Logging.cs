using UnityEngine;
using System;
using System.Runtime.InteropServices;

namespace Runity {
    public class Logging {
        private DLog m_cbLog;
        public Logging() {
            m_cbLog = CallbackLog;
            bind_log_callback(m_cbLog);
        }

        public void CallbackLog(LogMessage a_message) {
            switch(a_message.level) {
                case Level.Error:
                    UnityEngine.Debug.LogError("[RUST] ERROR: " + a_message.message);
                break;
                case Level.Warn:
                    UnityEngine.Debug.LogWarning("[RUST] WARN: " + a_message.message);
                break;
                case Level.Info:
                    UnityEngine.Debug.Log("[RUST] INFO: " + a_message.message);
                break;
                case Level.Debug:
                    UnityEngine.Debug.Log("[RUST] DEBUG: " + a_message.message);
                break;
                case Level.Trace:
                    UnityEngine.Debug.Log("[RUST] TRACE: " + a_message.message);
                break;
            } 
        }
        [DllImport("runity")] public static extern void bind_log_callback([MarshalAs(UnmanagedType.FunctionPtr)] DLog a_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)]
        public delegate void DLog(LogMessage a_message);
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct LogMessage {
        public Level level;
        public string message;
    }

    [System.Serializable]
    public enum Level: int {
        Off = 0,
        Error = 1,
        Warn,
        Info,
        Debug,
        Trace,
    }
}