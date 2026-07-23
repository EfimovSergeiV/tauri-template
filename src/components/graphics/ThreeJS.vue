<script setup>
  import { ref, onMounted } from 'vue'
  import PotPlantUrl from '../../assets/PotPlant.glb?url'

  import { 
    Scene, PerspectiveCamera, WebGLRenderer, 
    AmbientLight, DirectionalLight, Box3, Vector3, PointLight, HemisphereLight,
    PCFSoftShadowMap
  } from 'three'
  import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js'
  import { OrbitControls } from 'three/addons/controls/OrbitControls.js'

  let renderer, controls, model
  const experience = ref(null)

  const scene = new Scene()

  // свет
  const ambient = new AmbientLight(0x404040, 1)
  ambient.intensity = 2
  scene.add(ambient)

  const dirLight = new DirectionalLight(0xffffff, 1)
  dirLight.position.set(5, 10, 7.5)
  dirLight.intensity = 2
  scene.add(dirLight)

  const point1 = new PointLight(0xffffff, 0.8)
  point1.position.set(-5, 3, 5)
  point1.intensity = 1.5
  scene.add(point1)

  // точечный свет справа
  const point2 = new PointLight(0xffffff, 0.8)
  point2.position.set(5, 3, 5)
  scene.add(point2)

  // верхний "небо+земля"
  const hemiLight = new HemisphereLight(0xffffff, 0x444444, 0.6)
  hemiLight.position.set(0, 10, 0)
  hemiLight.intensity = 1
  scene.add(hemiLight)


  // камера
  const camera = new PerspectiveCamera(45, 1, 0.1, 1000)
  camera.position.set(0, 0, 4)
  scene.add(camera)

  // загрузка модели
  const gltfLoader = new GLTFLoader()
  gltfLoader.load(PotPlantUrl, (gltf) => {
    model = gltf.scene

    // нормализация размера в 250×250
    const box = new Box3().setFromObject(model)
    const size = new Vector3()
    box.getSize(size)
    const maxDim = Math.max(size.x, size.y, size.z)
    const scale = 2 / maxDim
    model.scale.setScalar(scale)

    // центрирование
    const center = box.getCenter(new Vector3())
    model.position.sub(center.multiplyScalar(scale))

    scene.add(model)
  }, undefined, (error) => {
    console.error('GLB load error:', error)
  })

  function updateRenderer() {
    renderer.setSize(400, 400)
    renderer.render(scene, camera)
    dirLight.castShadow = true
    renderer.shadowMap.enabled = true
    renderer.shadowMap.type = PCFSoftShadowMap
  }

  function setRenderer() {
    if (experience.value) {
      renderer = new WebGLRenderer({ canvas: experience.value, alpha: true, antialias: true })
      renderer.setClearColor(0x000000, 0) // прозрачный фон

      // OrbitControls
      controls = new OrbitControls(camera, renderer.domElement)
      controls.enableDamping = true
      controls.dampingFactor = 0.05
      controls.enableZoom = false // если зум не нужен
      updateRenderer()
    }
  }

  onMounted(() => {
    setRenderer()
    loop()
  })

  const loop = () => {
    if (model) {
      // автоповорот
      model.rotation.y += 0.005
    }

    if (controls) controls.update()
      updateRenderer()
      requestAnimationFrame(loop)
    }
</script>



<template>
  <div class="">
    <div class="flex items-center justify-center">
      <div class="">
        <canvas ref="experience" />
      </div>
    </div>
  </div>
</template>