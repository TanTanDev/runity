using System;
using System.Runtime.InteropServices;
using System.Collections.Generic; 

namespace Runity {
    // 

    public class World {
        // runity.dll callbacks
        private DSpawnPrefab m_cbSpawnPrefab;
        private DSpawnPrefabWithTransform m_cbSpawnPrefabWithTransform;
        private DSpawnPrefabWithTransformAndParent m_cbSpawnPrefabWithTransformAndParent;
        private DDespawnGameObject m_cbDespawnGameObject;
        private DUploadComponentTransform m_cbUploadComponentTransform;
        private DUploadComponentTextmeshui m_cbUploadComponentTextmeshui;
        private DKeyCallback m_cbKeyJustPressed;
        private DKeyCallback m_cbKeyJustReleased;
        private DKeyCallback m_cbKeyHeld;
        private DEntityTrackCollision m_cbEntityTrackCollision;

        // unity side data
        private Dictionary<string, UnityEngine.GameObject> m_prefabs;
        private Dictionary<UInt64, RustyGameObject> m_gameObjects;
        private Dictionary<int, System.Type> m_idToComponentType;

        public World(ScriptablePrefabs a_prefabBindings) {
            m_cbSpawnPrefab = SpawnPrefab;
            m_cbSpawnPrefabWithTransform = SpawnPrefabWithTransform;
            m_cbSpawnPrefabWithTransformAndParent = SpawnPrefabWithTransformAndParent;
            m_cbDespawnGameObject = DespawnGameObject;
            m_cbUploadComponentTransform = UploadComponentTransform;
            m_cbUploadComponentTextmeshui = UploadComponentTextmeshui;
            m_cbKeyJustPressed = InputKeyJustPressed;
            m_cbKeyJustReleased = InputKeyJustReleased;
            m_cbKeyHeld = InputKeyHeld;
            m_cbEntityTrackCollision = EntityTrackCollision;
            world_bind_spawn_prefab_callback(m_cbSpawnPrefab);
            world_bind_spawn_prefab_with_transform_callback(m_cbSpawnPrefabWithTransform);
            world_bind_spawn_prefab_with_transform_and_parent_callback(m_cbSpawnPrefabWithTransformAndParent);
            world_bind_despawn_gameobject_callback(m_cbDespawnGameObject);
            world_bind_upload_component_transform_callback(m_cbUploadComponentTransform);
            world_bind_upload_component_textmeshui_callback(m_cbUploadComponentTextmeshui);
            world_bind_input_key_just_pressed(m_cbKeyJustPressed);
            world_bind_input_key_just_released(m_cbKeyJustReleased);
            world_bind_input_key_held(m_cbKeyHeld);
            world_bind_entity_track_collision(m_cbEntityTrackCollision);
            m_prefabs = new Dictionary<string, UnityEngine.GameObject>();
            m_gameObjects = new Dictionary<UInt64, RustyGameObject>();
            for(int i = 0; i < a_prefabBindings.PrefabBindings.Length; i++) {
                var binding = a_prefabBindings.PrefabBindings[i];
                m_prefabs.Add(binding.RustName, binding.Prefab);
            }
        }

        public bool InputKeyJustPressed(UnityEngine.KeyCode a_key) {
            return UnityEngine.Input.GetKeyDown(a_key);
        }

        public bool InputKeyJustReleased(UnityEngine.KeyCode a_key) {
            return UnityEngine.Input.GetKeyUp(a_key);
        }

        public bool InputKeyHeld(UnityEngine.KeyCode a_key) {
            return UnityEngine.Input.GetKey(a_key);
        }

        public void DespawnGameObject(UInt64 a_identifierBits) {
            RustyGameObject gameObject;
            if(m_gameObjects.TryGetValue(a_identifierBits, out gameObject)) {
                UnityEngine.Object.Destroy(gameObject.GameObject);
                m_gameObjects.Remove(a_identifierBits);
            } else {
                UnityEngine.Debug.LogWarning("failed to despawn gameobject with id: " + a_identifierBits);
            }
        }

        public void SpawnPrefabWithTransform(string a_name, UInt64 a_identifierBits, CTransform a_transform) {
            SpawnPrefab(a_name, a_identifierBits);
            UploadComponentTransform(a_identifierBits, a_transform);
        }

        public void SpawnPrefabWithTransformAndParent(string a_name, UInt64 a_identifierBits, CTransform a_transform, UInt64 a_parentIdentifier) {
            SpawnPrefab(a_name, a_identifierBits);
            ParentTo(a_identifierBits, a_parentIdentifier);
            UploadComponentTransform(a_identifierBits, a_transform);
        }

        private void ParentTo(UInt64 a_childId, UInt64 a_parentId) {
            if(m_gameObjects.TryGetValue(a_childId, out var child)) {
                if(m_gameObjects.TryGetValue(a_parentId, out var parent)) {
                    child.GameObject.transform.SetParent(parent.GameObject.transform, true);
                } else {
                    UnityEngine.Debug.Log("no parent to bind child to " + a_childId + " " + a_parentId);
                }
            } else {
                UnityEngine.Debug.Log("no child we can parent");
            }
        }

        public void SpawnPrefab(string a_name, UInt64 a_identifierBits) {
            UnityEngine.GameObject prefab;
            if(m_prefabs.TryGetValue(a_name, out prefab)) {
                UnityEngine.GameObject spawnedGO = UnityEngine.GameObject.Instantiate(prefab);  
                RustEntityComponent rustEntity = spawnedGO.AddComponent<RustEntityComponent>();
                rustEntity.IdentifierBits = a_identifierBits;
                var rustyGameObject = new RustyGameObject(spawnedGO);
                if(m_gameObjects.ContainsKey(a_identifierBits)) {
                    UnityEngine.Debug.LogError("id already exists: {}"+ a_identifierBits);
                    return;
                }
                m_gameObjects.Add(a_identifierBits, rustyGameObject);
            } else {
                UnityEngine.Debug.LogError("fail spawn prefab with name: " + a_name);
            }
        }

        public void UploadComponentTextmeshui(UInt64 a_identifierBits, string a_text) {
            //UnityEngine.Debug.Log(a_identifierBits + " tried to upload text: " + a_text);
            if(m_gameObjects.TryGetValue(a_identifierBits, out var gameObject)) {
                var text = gameObject.GetComponentFromId<TMPro.TextMeshProUGUI>(3);
                text.text = a_text;
            }
        }

        public void UploadComponentTransform(UInt64 a_identifierBits, CTransform a_transform) {
            RustyGameObject gameObject;
            if(m_gameObjects.TryGetValue(a_identifierBits, out gameObject)) {
                // todo, fetch id of component, not hardcode transform to 0
                var transform = gameObject.GetComponentFromId<UnityEngine.Transform>(0);
                UnityEngine.Vector3 position = new UnityEngine.Vector3(a_transform.x, a_transform.y, a_transform.z); 
                transform.position = position;
                // I'm to lazy to implement RectTransform AAAAAAH
                var rectTransform = gameObject.GetComponentFromId<UnityEngine.RectTransform>(2);
                if(rectTransform != null)
                    rectTransform.anchoredPosition = position;
            } else {
            //    UnityEngine.Debug.LogWarning("failed to upload transform to gameobject(No such entity) with id: " + a_identifier);
            }
        }

        public void EntityTrackCollision(UInt64 a_identifierBits) {
            RustyGameObject rustyGameObject;
            if(m_gameObjects.TryGetValue(a_identifierBits, out rustyGameObject)) {
                var collisionTracker = rustyGameObject.GameObject.AddComponent<CollisionTrackerComponent>();
                collisionTracker.OwnerEntityIdBits = a_identifierBits;
                collisionTracker.CollisionEvents = new Queue<CollisionEvent>();
                UnityEngine.Debug.Log("added collision tracker, data = " + collisionTracker);
                //var transform = gameObject.GetComponentFromId<UnityEngine.Transform>(0);
                //transform.position = a_transform.ToVector3();
                //transform.position = new UnityEngine.Vector3(a_transform.x, a_transform.y, a_transform.z);//a_transform.y, a_transform.z);
            } else {
                UnityEngine.Debug.Log("failed to add entity track colision entity does not exist");
            //    UnityEngine.Debug.LogWarning("failed to upload transform to gameobject(No such entity) with id: " + a_identifier);
            }
        }

        // todo, move to seperate file
        [StructLayout(LayoutKind.Sequential)]
        public struct CTransform {
            //public CVector position; 
            public float x;
            public float y;
            public float z;
            //public System.Int16 z;
            //public UnityEngine.Vector3 ToVector3() {
            //    return new UnityEngine.Vector3(x,y,z);
            //}
        }

        //[StructLayout(LayoutKind.Sequential)]
        //public struct CVector {
        //    public float x;
        //    public float y;
        //    public float z;
        //    public UnityEngine.Vector3 ToVector3() {
        //        return new UnityEngine.Vector3(x,y,z);
        //    }
        //}

        public void DownloadEntityCollisionEvents() {
            foreach(var entry in m_gameObjects) {
                UInt64 entityId = entry.Key;
                RustyGameObject rustyGameObject = entry.Value;
                var collisionTracker = rustyGameObject.GetComponentFromId<CollisionTrackerComponent>(1);
                if(collisionTracker != null) {
                    foreach(CollisionEvent collisionEvent in collisionTracker.CollisionEvents) {
                        world_download_entity_collision_event(entityId, collisionEvent);
                    }
                    collisionTracker.CollisionEvents.Clear();
                }
            }
        }

        [DllImport("runity")] public static extern void world_bind_spawn_prefab_callback([MarshalAs(UnmanagedType.FunctionPtr)] DSpawnPrefab a_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate void DSpawnPrefab(string a_name, UInt64 a_identifierBits);
        [DllImport("runity")] public static extern void world_bind_spawn_prefab_with_transform_callback([MarshalAs(UnmanagedType.FunctionPtr)] DSpawnPrefabWithTransform a_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate void DSpawnPrefabWithTransform(string a_name, UInt64 a_identifierBits, CTransform a_transform);
        [DllImport("runity")] public static extern void world_bind_spawn_prefab_with_transform_and_parent_callback([MarshalAs(UnmanagedType.FunctionPtr)] DSpawnPrefabWithTransformAndParent a_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate void DSpawnPrefabWithTransformAndParent(string a_name, UInt64 a_identifierBits, CTransform a_transform, UInt64 a_parentIdentifier);
        [DllImport("runity")] public static extern void world_bind_despawn_gameobject_callback([MarshalAs(UnmanagedType.FunctionPtr)] DDespawnGameObject a_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate void DDespawnGameObject(UInt64 a_identifierBits);
        [DllImport("runity")] public static extern void world_bind_upload_component_transform_callback([MarshalAs(UnmanagedType.FunctionPtr)] DUploadComponentTransform a_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate void DUploadComponentTransform(UInt64 a_identifierBits, CTransform a_transfrom);

        [DllImport("runity")] public static extern void world_bind_upload_component_textmeshui_callback([MarshalAs(UnmanagedType.FunctionPtr)] DUploadComponentTextmeshui a_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate void DUploadComponentTextmeshui(UInt64 a_identifierBits, string a_text);

        [DllImport("runity")] public static extern void world_bind_input_key_just_pressed([MarshalAs(UnmanagedType.FunctionPtr)] DKeyCallback a_callback);
        [DllImport("runity")] public static extern void world_bind_input_key_just_released([MarshalAs(UnmanagedType.FunctionPtr)] DKeyCallback a_callback);
        [DllImport("runity")] public static extern void world_bind_input_key_held([MarshalAs(UnmanagedType.FunctionPtr)] DKeyCallback a_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate bool DKeyCallback(UnityEngine.KeyCode a_key);
        [DllImport("runity")] public static extern void world_bind_entity_track_collision([MarshalAs(UnmanagedType.FunctionPtr)] DEntityTrackCollision a_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate void DEntityTrackCollision(UInt64 a_identifierBits);

        [DllImport("runity")] public static extern void world_download_entity_collision_event(UInt64 a_entityIdBits, CollisionEvent a_collisionEvent);
    }
}