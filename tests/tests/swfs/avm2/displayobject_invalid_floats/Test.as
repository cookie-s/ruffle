package {
    import flash.display.MovieClip;
    import flash.geom.Matrix;

    public class Test extends MovieClip {
        private var props = ["rotation", "x", "y", "scaleX", "scaleY"];

        public function Test() {
            var a = new MovieClip();
            a.rotation = 26.56;
            a.scaleX = 2.2;
            a.scaleY = -4.1;
            a.x = 5;
            a.y = 6;
            // printChange(a, "scaleY", 4.1);

            var m = new Matrix();
            m.scale(2.2, -4.1);
            m.rotate(26.56 * Math.PI / 180);
            m.translate(5, 6);
            // m.concat(skewMatrix(1, 2));
            trace(a.transform.matrix);
            trace(m);
            // FIXME - we should also be testing Infinity and -Infinity here,
            // but those give very weird values back in the matrix,
            // and I havne't yet figured out how to reproduce them. Hopefully,
            // there are no SWFs relying on the behavior.

            var limits = [0, NaN];
            for each (var prop in props) {
                trace("// " + prop + " = NaN");
                for each (var l1 in limits) {
                    for each (var l2 in limits) {
                        for each (var l3 in limits) {
                            for each (var l4 in limits) {
                                var clip = new MovieClip();
                                clip.transform.matrix = new Matrix(l1, l2, l3, l4, 0, 0);
                                printChange(clip, prop, 0);
                                trace("");
                            }
                        }
                    }
                }
            }

            var limits = [1, NaN];
            for each (var prop in props) {
                trace("// " + prop + " = NaN");
                for each (var l1 in limits) {
                    for each (var l2 in limits) {
                        for each (var l3 in limits) {
                            for each (var l4 in limits) {
                                var clip = new MovieClip();
                                clip.transform.matrix = new Matrix(l1, l2, l3, l4, 0, 0);
                                printChange(clip, prop, 0);
                                trace("");
                            }
                        }
                    }
                }
            }
            return;

            var limits = [0, NaN, Infinity, -Infinity];
            for each (var prop in props) {
                trace("// " + prop + " = NaN");
                for each (var l1 in limits) {
                    for each (var l2 in limits) {
                        for each (var l3 in limits) {
                            for each (var l4 in limits) {
                                var clip = new MovieClip();
                                clip.transform.matrix = new Matrix(l1, l2, l3, l4, 0, 0);
                                printChange(clip, prop, 0);
                                trace("");
                            }
                        }
                    }
                }
            }
            return;

            for each (var prop in props) {
                trace("// " + prop + " = Infinity");
                var clip = new MovieClip();
                clip.transform.matrix = new Matrix(2, 0, 4, 0, 5, 6);
                printChange(clip, prop, Infinity);

                clip = new MovieClip();
                var newMat = new Matrix(2, 1, 4, 1, 5, 6);
                clip.transform.matrix = newMat;
                printChange(clip, prop, Infinity);

                clip = new MovieClip();
                var newMat = new Matrix(7, 0, 9, 0, 11, 12);
                clip.transform.matrix = newMat;
                printChange(clip, prop, Infinity);

                trace("");
                trace("");
            }
        }

        private function skewMatrix(skew_x: Number, skew_y: Number):Matrix {
            return new Matrix(0, Math.tan(skew_y), Math.tan(skew_x), 1)
        }

        private function printChange(clip:MovieClip, prop:String, value:*) {
            var result = "";
            for each (var p in props) {
                result += p + "=" + clip[p] + ", ";
            }
            trace("  " + result);
            trace("  transform.matrix = " + clip.transform.matrix);

            trace("clip[" + prop + "] = " + value);
            clip[prop] = value;

            result = "";
            for each (var p in props) {
                result += p + "=" + clip[p] + ", ";
            }
            trace("  " + result);
            trace("  transform.matrix = " + clip.transform.matrix);
        }
    }
}
